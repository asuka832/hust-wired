<h1 align="center">华科有线网</h1>

<p align="center">
  插上网线，自动完成华中科技大学校园有线网认证。
</p>

<p align="center">
  <a href="https://github.com/asuka832/hust-wired/releases"><img alt="release" src="https://img.shields.io/github/v/release/asuka832/hust-wired?style=flat-square&color=3b82f6"></a>
  <img alt="Windows" src="https://img.shields.io/badge/Windows-x64-0078D4?style=flat-square&logo=windows&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.96+-DEA584?style=flat-square&logo=rust&logoColor=white">
  <a href="LICENSE"><img alt="license" src="https://img.shields.io/badge/license-MIT-22c55e?style=flat-square"></a>
  <img alt="HUST" src="https://img.shields.io/badge/%E5%8D%8E%E4%B8%AD%E7%A7%91%E6%8A%80%E5%A4%A7%E5%AD%A6-%E6%9C%89%E7%BA%BF%E7%BD%91-c41e3a?style=flat-square">
</p>

<p align="center">
  <b>简体中文</b> · <a href="README_EN.md">English</a>
</p>

---

面向华中科技大学锐捷 ePortal 校园有线网的 Rust 命令行工具。程序会先并发探测当前网络状态，仅在停留在认证页时提交登录。

> [!NOTE]
> 非官方工具，请使用自己的校园网账号。切勿将账号、密码或完整登录网址贴到聊天记录或 Issue 中。

## 特性

- **开箱即用**：单文件 Rust 可执行程序，下载解压即可运行，无需安装 Rust 或 Python
- **简洁菜单**：首次启动直达账号配置与登录；菜单按终端宽度自动排布，窄窗口自动收窄
- **持续监测**：定时检测网络并在掉线后自动重连，随时按 `Q` 停止并返回菜单
- **开机自启**：支持注册 Windows 当前用户开机自启（`HKCU\...\Run`），登录后自动在后台监测
- **本机加密**：密码通过 Windows DPAPI 加密保存在当前用户配置目录，不读取浏览器密码

## 快速开始

前往 [Releases](https://github.com/asuka832/hust-wired/releases/latest) 下载 `hust-wireless-rs-windows-x64.zip`，解压后双击 `hust.cmd`。

1. **首次配置**：选择 `1`「设置账号并连接」，输入校园网账号和密码。保存后会立即检测网络，需要认证时自动登录。
2. **开机自启**：登录成功后，在主菜单选择 `5` 即可启用开机自启。如果账号需要填写运营商代码，按 `M` 进入更多设置再选 `4`。
3. **手动补充登录网址（仅在需要时）**：如果程序无法自动取得认证参数，会提示你粘贴浏览器地址栏里的整段网址——**要还没登录成功的那一页**。直接回车可取消，无效地址不会覆盖原有配置。

## 交互菜单

### 主菜单

| 按键 | 功能 | 说明 |
| --- | --- | --- |
| `1` | 立即登录 | 检测网络状态，需要认证时提交登录 |
| `2` | 检查网络 | 并发探测连通性并显示耗时（最长约 3 秒，按 `Q` 取消） |
| `5` | 启用开机自启 | 写入 Windows 开机自启项，登录后自动监测 |
| `M` | 更多设置 | 展开诊断、监测、账号与清理选项 |
| `0` | 退出 | 退出程序 |

### 更多设置

| 按键 | 功能 | 说明 |
| --- | --- | --- |
| `3` | 参数诊断 | 检查网络并打印认证参数字段名（不输出参数值） |
| `4` | 设置账号 | 重新配置账号、密码与运营商代码 |
| `6` | 取消开机自启 | 移除 Windows 开机自启项 |
| `7` | 持续监测 | 在当前窗口每 20 秒检测一次并自动重连，按 `Q` 返回菜单 |
| `8` | 卸载工具 | 清除自启项、账号、日志和本地登录地址（需输入 `y` 确认） |
| `9` | 更新登录地址 | 手动粘贴并保存认证页整段网址 |
| `0` | 返回 | 返回主菜单 |

## 命令行用法

也可以跳过菜单，直接通过子命令调用：

```powershell
.\hust-wireless-rs.exe setup       # 设置账号、密码和运营商代码
.\hust-wireless-rs.exe login       # 立即认证一次
.\hust-wireless-rs.exe status      # 检查网络状态与延迟
.\hust-wireless-rs.exe diagnose    # 显示认证参数字段名
.\hust-wireless-rs.exe watch       # 持续监测并自动重连（按 Q 退出）
.\hust-wireless-rs.exe install     # 启用开机自启
.\hust-wireless-rs.exe uninstall   # 仅取消开机自启
.\hust-wireless-rs.exe set-url     # 保存登录页的整段网址
.\hust-wireless-rs.exe remove      # 清除自启项、账号、日志和登录地址（保留程序文件）
```

> `uninstall` 只会取消开机自启；`remove` 会清理全部本机数据但不删除程序本身，随后直接删除解压文件夹即可彻底卸载。

## 常见问题

- **无法获取认证参数 / 以前能用后来失败**：在更多设置中选 `3` 查看诊断字段（正常跳转会包含 `nasip`、`wlanuserip` 等字段）。若网络或设备地址发生变化，选 `9` 重新粘贴认证页地址栏里的整段网址即可。
- **提示验证码或设备未注册**：需要先在浏览器中完成对应处理，再回到工具登录。
- **持续监测自动停止**：认证连续失败 5 次后，监测会自动退避并停止，避免反复提交错误凭据；请检查账号密码或登录地址后重新启动。

## 本机数据

所有配置和日志仅保存在当前 Windows 用户的配置目录中：

```text
%APPDATA%\hust-wireless-cli\
├── config.json          # 账号与运营商代码，密码为 DPAPI 密文
├── wired-login-url.txt  # 本地保存的登录网址（明文）
└── hust-auth.log        # 监测与认证日志
```

## 从源码构建

需要 Rust 稳定版工具链：

```powershell
cd .\hust-wireless-rs
cargo build --release
```

如果没有安装 Visual Studio C++ 链接器，可使用 GNU 工具链（需将 64 位 MinGW 的 `bin` 目录加入 `PATH`）：

```powershell
cd .\hust-wireless-rs
cargo +stable-x86_64-pc-windows-gnu build --release
```

编译完成后，运行 `target\release\hust-wireless-rs.exe` 或直接双击 `hust-wireless-rs\hust.cmd`。

运行单元测试：

```powershell
cd .\hust-wireless-rs
cargo test
```

## 技术与安全说明

- **协议限制**：学校认证入口为 HTTP，RSA 公钥同样通过该 HTTP 链路下发，不等同于 HTTPS。若认证页未启用密码加密，程序会拒绝以明文提交密码并中止登录。
- **本地存储**：DPAPI 仅保护磁盘上的静态密码，同一 Windows 用户下的其他进程仍可能调用系统接口解密；`wired-login-url.txt` 包含设备与会话参数，请勿外泄。
- **连通性探测**：并发探测小米、华为的 `generate_204`、`123.123.123.123` 以及校园认证页 `index.jsp`，单次超时约 3 秒。
- **平台支持**：开机自启功能仅支持 Windows。

## 许可证

[MIT](LICENSE)
