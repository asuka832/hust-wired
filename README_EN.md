<h1 align="center">HUST Wired</h1>

<p align="center">
  Plug in the cable. Automatically sign in to the HUST campus wired network.
</p>

<p align="center">
  <a href="https://github.com/asuka832/hust-wired/releases"><img alt="release" src="https://img.shields.io/github/v/release/asuka832/hust-wired?style=flat-square&color=3b82f6"></a>
  <img alt="Windows" src="https://img.shields.io/badge/Windows-x64-0078D4?style=flat-square&logo=windows&logoColor=white">
  <img alt="Rust" src="https://img.shields.io/badge/Rust-1.96+-DEA584?style=flat-square&logo=rust&logoColor=white">
  <a href="LICENSE"><img alt="license" src="https://img.shields.io/badge/license-MIT-22c55e?style=flat-square"></a>
  <img alt="HUST" src="https://img.shields.io/badge/HUST-wired%20network-c41e3a?style=flat-square">
</p>

<p align="center">
  <a href="README.md">简体中文</a> · <b>English</b>
</p>

---

A Rust command-line tool for the Huazhong University of Science and Technology (HUST) campus wired network on Ruijie ePortal. It probes connectivity concurrently and submits a login only when the captive portal is waiting.

> [!NOTE]
> Unofficial tool. Use your own campus account. Never post your account, password, or full login URL in chats or GitHub issues.

## Features

- **Ready to run**: Single Rust binary—download and extract without installing Rust or Python.
- **Adaptive menu**: Streamlined first-run setup; the numbered menu automatically adapts to narrow terminals.
- **Auto-reconnect watch**: Periodically checks connectivity and signs back in after a drop. Press `Q` anytime to stop.
- **Windows startup**: Optional per-user startup entry (`HKCU\...\Run`) to monitor and authenticate in the background after Windows sign-in.
- **Local encryption**: Passwords are encrypted with Windows DPAPI under your user profile. The tool never reads browser passwords.

## Quick Start

Download `hust-wireless-rs-windows-x64.zip` from [Releases](https://github.com/asuka832/hust-wired/releases/latest), extract it, and double-click `hust.cmd`.

1. **First launch**: Choose `1` to save your campus account and password. The tool immediately checks the network and signs in if needed.
2. **Start with Windows**: Choose `5` from the main menu to enable automatic sign-in at Windows startup. If your account requires an operator code, press `M` for more settings and choose `4`.
3. **Manual login URL (only if prompted)**: If the tool cannot obtain portal parameters automatically, paste the entire address from your browser bar—**use the page before login succeeds**. Press Enter to cancel; an invalid URL will not overwrite your saved configuration.

## Interactive Menu

### Main Menu

| Key | Action | Description |
| --- | --- | --- |
| `1` | Sign in now | Check the network and submit login if authentication is required |
| `2` | Check network | Probe connectivity concurrently and report latency (~3 s max, press `Q` to cancel) |
| `5` | Enable startup | Register Windows per-user startup to watch in the background |
| `M` | More settings | Open diagnostics, watch mode, account settings, and cleanup options |
| `0` | Exit | Exit the program |

### More Settings

| Key | Action | Description |
| --- | --- | --- |
| `3` | Diagnose parameters | Check network and print portal query field names (values are never printed) |
| `4` | Set account | Update campus account, password, and optional operator code |
| `6` | Disable startup | Remove the Windows startup entry |
| `7` | Watch in window | Check every 20 s and auto-reconnect in this window; press `Q` to return to menu |
| `8` | Remove local data | Clear startup entry, account, log, and saved login URL (`y` confirmation required) |
| `9` | Update login URL | Manually paste and save the full portal login URL |
| `0` | Back | Return to the main menu |

## CLI Usage

You can also invoke subcommands directly without entering the menu:

```powershell
.\hust-wireless-rs.exe setup       # Configure account, password, and operator code
.\hust-wireless-rs.exe login       # Sign in once if needed
.\hust-wireless-rs.exe status      # Check network state and latency
.\hust-wireless-rs.exe diagnose    # Print portal parameter field names
.\hust-wireless-rs.exe watch       # Watch and auto-reconnect (press Q to exit)
.\hust-wireless-rs.exe install     # Enable Windows startup
.\hust-wireless-rs.exe uninstall   # Disable Windows startup only
.\hust-wireless-rs.exe set-url     # Save the full portal login URL
.\hust-wireless-rs.exe remove      # Clear startup, account, log, and URL (keeps binaries)
```

> `uninstall` only removes the startup entry. `remove` clears all local data while leaving the program files intact—delete the extracted folder afterward to uninstall completely.

## Troubleshooting

- **Missing parameters / stopped working later**: Choose `3` under more settings to inspect field names (a normal redirect includes fields such as `nasip` and `wlanuserip`). If your network or device address changed, choose `9` and paste the full portal URL from your browser bar.
- **Captcha or unregistered device**: Complete the prompt in a web browser first, then return to the tool.
- **Watch stopped automatically**: After 5 consecutive authentication failures, watch mode backs off and stops to avoid repeatedly submitting invalid credentials. Verify your account or login URL before restarting.

## Local Data

All local settings and logs are stored under your Windows user profile:

```text
%APPDATA%\hust-wireless-cli\
├── config.json          # account & operator code, password encrypted with DPAPI
├── wired-login-url.txt  # saved portal login URL (plain text)
└── hust-auth.log        # watch & authentication log
```

## Build from Source

Rust stable is required:

```powershell
cd .\hust-wireless-rs
cargo build --release
```

Without the Visual Studio C++ linker, use the GNU target (requires a 64-bit MinGW `bin` directory on `PATH`):

```powershell
cd .\hust-wireless-rs
cargo +stable-x86_64-pc-windows-gnu build --release
```

After building, run `target\release\hust-wireless-rs.exe` or double-click `hust-wireless-rs\hust.cmd`.

Run unit tests:

```powershell
cd .\hust-wireless-rs
cargo test
```

## Technical & Security Notes

- **HTTP portal**: The campus portal runs over plain HTTP, and the RSA public key is fetched over the same link; this is not equivalent to HTTPS. If the portal disables password encryption, the tool refuses to send a plaintext password and aborts.
- **Local storage**: DPAPI protects the password at rest on disk, though other processes running under the same Windows user can still decrypt it. `wired-login-url.txt` contains device and session parameters—do not share it.
- **Connectivity probes**: Probes Xiaomi and Huawei `generate_204` endpoints, `123.123.123.123`, and the campus portal `index.jsp` concurrently, timing out after ~3 seconds.
- **Platform support**: Windows startup integration is supported on Windows only.

## License

[MIT](LICENSE)
