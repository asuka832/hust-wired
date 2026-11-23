use terminal_size::{Width, terminal_size};
use unicode_width::UnicodeWidthStr;

pub const MENU_OPTIONS: &[(&str, &str, &str)] = &[
    ("1", "login", "立即登录"),
    ("2", "status", "检查网络"),
    ("3", "diagnose", "参数诊断"),
    ("4", "setup", "设置账号"),
    ("5", "install", "启用开机自启"),
    ("6", "uninstall", "取消开机自启"),
    ("7", "watch", "持续监测"),
    ("8", "remove", "卸载工具"),
    ("9", "set-url", "更新登录地址"),
];

pub fn is_main_option(choice: &str) -> bool {
    matches!(choice, "1" | "2" | "5")
}

pub fn menu_lines(width: usize, more: bool) -> Vec<String> {
    let mut items: Vec<String> = MENU_OPTIONS
        .iter()
        .filter(|(number, _, _)| is_main_option(number) != more)
        .map(|(number, _, label)| format!("[{number}] {label}"))
        .collect();
    if !more {
        items.push("[M] 更多设置".to_string());
    }
    items.push(if more { "[0] 返回" } else { "[0] 退出" }.to_string());
    let cell_width = items
        .iter()
        .map(|item| UnicodeWidthStr::width(item.as_str()))
        .max()
        .unwrap_or(0)
        + 4;
    let columns = (width / cell_width).clamp(1, 3);

    items
        .chunks(columns)
        .map(|row| {
            let mut line = String::from("  ");
            for item in row {
                line.push_str(item);
                let padding = cell_width - UnicodeWidthStr::width(item.as_str());
                line.extend(std::iter::repeat_n(' ', padding));
            }
            line.trim_end().to_string()
        })
        .collect()
}

pub fn terminal_width() -> usize {
    terminal_size()
        .map(|(Width(width), _)| width as usize)
        .unwrap_or(80)
}

pub fn find_option(choice: &str) -> Option<&'static str> {
    MENU_OPTIONS
        .iter()
        .find(|(number, _, _)| *number == choice)
        .map(|(_, command, _)| *command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_row_contains_three_options() {
        let lines = menu_lines(120, false);
        assert!(lines[0].contains("[1]"));
        assert!(lines[0].contains("[2]"));
        assert!(lines[0].contains("[5]"));
    }

    #[test]
    fn narrow_terminal_uses_one_column() {
        assert_eq!(menu_lines(20, true).len(), 7);
    }
}
