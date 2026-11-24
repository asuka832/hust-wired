mod autostart;
mod config;
mod dpapi;
mod encoding;
mod error;
mod login_url;
mod menu;
mod portal;
mod rsa;
mod watch;

use std::io::{self, IsTerminal, Write};
use std::time::Instant;

use error::{AppError, Result};
use portal::{PORTAL_URL, Portal};

const COMMANDS: &[&str] = &[
    "setup",
    "login",
    "status",
    "diagnose",
    "watch",
    "install",
    "uninstall",
    "remove",
    "set-url",
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match run(&args) {
        Ok(()) => 0,
        Err(AppError::Cancelled) => 0,
        Err(AppError::Message(message) | AppError::LoginUrlRequired(message)) => {
            eprintln!("错误：{message}");
            if args.first().is_some_and(|value| value == "login") && message.starts_with("认证失败")
            {
                eprintln!("请检查账号密码。若设备或网络地址已变化，可用 set-url 更新登录地址。");
            }
            1
        }
        Err(error) => {
            eprintln!("错误：{error}");
            1
        }
    };
    std::process::exit(code);
}

fn run(args: &[String]) -> Result<()> {
    if args.len() == 1 && matches!(args[0].as_str(), "-h" | "--help" | "help") {
        print_help();
        return Ok(());
    }
    match args {
        [] => run_menu(),
        [command] if COMMANDS.contains(&command.as_str()) => run_command(command, None),
        [command, flag] if command == "watch" && matches!(flag.as_str(), "--quiet" | "-q") => {
            run_command("watch", Some(false))
        }
        [command, ..] => Err(AppError::msg(format!(
            "无效命令：{command}\n可用命令：{}",
            COMMANDS.join(", ")
        ))),
    }
}

fn print_help() {
    println!("华科有线网命令行自动认证");
    println!();
    println!("用法：hust-wireless-rs [命令]");
    println!();
    println!("命令：");
    println!("  setup       设置账号和密码");
    println!("  login       立即认证一次");
    println!("  status      检查网络");
    println!("  diagnose    显示认证参数字段名");
    println!("  watch       持续监测并自动重连");
    println!("  install     启用开机自启");
    println!("  uninstall   取消开机自启");
    println!("  remove      清除账号、日志和自启项");
    println!("  set-url     保存登录页的整段网址");
    println!();
    println!("不指定命令时进入交互菜单。");
}

fn run_command(command: &str, interactive: Option<bool>) -> Result<()> {
    let interactive = interactive.unwrap_or_else(|| io::stdin().is_terminal());
    match command {
        "setup" => setup(true),
        "install" => {
            autostart::install()?;
            println!("已设置登录 Windows 后自动监测。");
            Ok(())
        }
        "uninstall" => {
            autostart::uninstall()?;
            println!("已取消开机自启。");
            Ok(())
        }
        "set-url" => {
            prompt_login_url(PORTAL_URL)?;
            Ok(())
        }
        "remove" => remove(),
        "status" | "diagnose" => status(command == "diagnose"),
        "login" | "watch" => {
            let portal = Portal::new(PORTAL_URL)?;
            let credentials = config::load_credentials()?;
            if command == "login" {
                if interactive {
                    println!("正在检查网络并尝试认证…");
                }
                let message = if interactive {
                    login_with_guidance(&portal, &credentials)?
                } else {
                    portal::login_once(&portal, &credentials)?
                };
                println!("{message}");
            } else {
                watch::watch(&portal, &credentials, !interactive)?;
            }
            Ok(())
        }
        _ => unreachable!(),
    }
}

fn run_menu() -> Result<()> {
    let mut more = false;
    loop {
        let first_run = !config::config_path().try_exists()?;
        if first_run {
            more = false;
            println!("\n华科有线网认证\n  [1] 设置账号并连接\n  [0] 退出");
        } else {
            println!(
                "\n{}",
                if more {
                    "更多设置"
                } else {
                    "华科有线网认证"
                }
            );
            println!(
                "{}",
                menu::menu_lines(menu::terminal_width(), more).join("\n")
            );
        }
        let choice = match read_line_optional("请输入选项：")? {
            Some(choice) => choice,
            None => {
                println!("\n已退出。");
                return Ok(());
            }
        };
        let choice = choice.trim();
        if choice == "0" {
            if more {
                more = false;
                continue;
            }
            return Ok(());
        }
        if !first_run && !more && choice.eq_ignore_ascii_case("m") {
            more = true;
            continue;
        }
        let result = if first_run && choice == "1" {
            setup(false).and_then(|()| run_command("login", Some(true)))
        } else if !first_run
            && menu::is_main_option(choice) != more
            && let Some(command) = menu::find_option(choice)
        {
            run_command(command, Some(true))
        } else {
            println!("无效选项，请输入菜单中显示的按键。");
            continue;
        };
        match result {
            Ok(()) => {}
            Err(AppError::Cancelled) => {
                watch::clear_stop();
                println!("\n已停止当前操作，返回菜单。");
            }
            Err(error) => eprintln!("错误：{error}"),
        }
    }
}

fn setup(ask_service: bool) -> Result<()> {
    println!("认证服务器使用 HTTP，公钥来源无法验证。");
    let username = read_line("校园网账号：")?.trim().to_string();
    if username.is_empty() {
        return Err(AppError::msg("账号不能为空"));
    }
    let password = rpassword::prompt_password("校园网密码：")?;
    if password.is_empty() {
        return Err(AppError::msg("密码不能为空"));
    }
    let service = if ask_service {
        read_line("运营商代码（通常直接回车）：")?
            .trim()
            .to_string()
    } else {
        String::new()
    };
    config::save_credentials(&username, &password, &service)?;
    println!("已保存账号。");
    Ok(())
}

fn status(diagnose: bool) -> Result<()> {
    let portal = Portal::new(PORTAL_URL)?;
    println!(
        "正在检测中（最长约 {} 秒，按 Q 取消）…",
        portal::NETWORK_CHECK_TIMEOUT.as_secs()
    );
    watch::clear_stop();
    let started = Instant::now();
    let check = portal.check_network();
    if watch::stop_requested() {
        watch::clear_stop();
        println!("已取消。");
        return Ok(());
    }
    let elapsed = started.elapsed().as_millis();
    println!("{}（延迟 {elapsed} ms）", check.state.label());
    if diagnose {
        let fields = login_url::query_field_names(&check.query);
        println!(
            "认证参数字段：{}",
            if fields.is_empty() {
                "无".to_string()
            } else {
                fields.join(", ")
            }
        );
        match login_url::load_login_query(&portal.base_url) {
            Ok(query) => println!(
                "本地登录地址字段：{}",
                login_url::query_field_names(&query).join(", ")
            ),
            Err(error) => println!("本地登录地址：{error}"),
        }
    }
    Ok(())
}

fn login_with_guidance(portal: &Portal, credentials: &config::Credentials) -> Result<String> {
    match portal::login_once(portal, credentials) {
        Ok(message) => Ok(message),
        Err(AppError::LoginUrlRequired(message)) => {
            println!("{message}");
            if !prompt_login_url(&portal.base_url)? {
                return Ok("本次登录已取消。".to_string());
            }
            portal::login_once(portal, credentials)
        }
        Err(error) => Err(error),
    }
}

fn prompt_login_url(portal_base: &str) -> Result<bool> {
    println!("把浏览器地址栏里的整段网址粘贴到下面。");
    println!("要还没登录成功的那一页。登录成功后的网址不能用，也不要发到聊天或 Issue。");
    loop {
        let url = read_line("登录页网址（直接回车取消）：")?;
        let url = url.trim();
        if url.is_empty() {
            println!("已取消，原有登录地址保持不变。");
            return Ok(false);
        }
        match login_url::save_login_url(url, portal_base) {
            Ok(_) => {
                println!("已保存到当前 Windows 用户的配置目录，下次会自动复用。");
                return Ok(true);
            }
            Err(error) => println!("{error}"),
        }
    }
}

fn remove() -> Result<()> {
    println!("将清除开机自启、账号配置、日志和本地登录地址。项目源码会保留。");
    if !confirm_action("卸载工具")? {
        println!("已取消。");
        return Ok(());
    }
    autostart::uninstall()?;
    config::remove_config()?;
    let login_path = config::login_url_path();
    match std::fs::remove_file(&login_path) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    config::remove_log_with_retry()?;
    config::remove_empty_config_dir();
    println!("本地数据已清理；删除项目文件夹即可移除程序文件。");
    Ok(())
}

fn confirm_action(label: &str) -> Result<bool> {
    loop {
        let answer = read_line(&format!("确定{label}？(y/n)："))?;
        match answer.trim().to_ascii_lowercase().as_str() {
            "y" => return Ok(true),
            "n" => return Ok(false),
            _ => println!("请输入 y 或 n。"),
        }
    }
}

fn read_line(prompt: &str) -> Result<String> {
    read_line_optional(prompt)?.ok_or_else(|| AppError::msg("输入已结束"))
}

fn read_line_optional(prompt: &str) -> Result<Option<String>> {
    print!("{prompt}");
    io::stdout().flush()?;
    let mut value = String::new();
    let read = io::stdin().read_line(&mut value)?;
    if read == 0 {
        return Ok(None);
    }
    Ok(Some(value.trim_end_matches(['\r', '\n']).to_string()))
}
