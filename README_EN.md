<h1 align="center">HUST Wired</h1>

<p align="center">
Plug in the cable. Sign in to the campus wired network.
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

A Windows command-line tool for the HUST campus wired network on Ruijie ePortal. It checks whether authentication is required, and submits a login only when the portal page is waiting.

> Unofficial. Use your own campus account. Do not post your account, password, or full login URL in an issue.

## Features

- Run it from the release archive. Rust is not required.
- A numbered menu that wraps when the terminal is narrow.
- A watch loop that signs in again after a drop. Press `Q` to leave it.
- Optional sign-in check after Windows starts.
- The password stays on this PC, protected with Windows DPAPI. The program does not read browser passwords.

## Download

Get `hust-wireless-rs-windows-x64.zip` from [Releases](https://github.com/asuka832/hust-wired/releases/latest) and double-click `hust.cmd`.

1. On first launch, choose `1` and enter your campus account and password. The tool saves them, checks the network, and attempts to sign in if needed.
2. Choose `5` from the main menu to start automatically at Windows sign-in. If your account needs an operator code, press `M` for more settings, then choose `4`.
3. If the tool cannot fetch the portal parameters, it prompts for the full `index.jsp?` URL from the page with the green Login button. Press Enter to cancel. A bad URL does not replace a saved one.

## Menu

- Main menu: `1` sign in, `2` check the network, `5` start at Windows sign-in, `M` more settings, `0` exit.
- More settings: `3` show portal field names, `4` update the account, `6` disable startup, `7` watch in this window, `8` remove local data, `9` update the login URL, `0` go back. Removing data requires a `y` confirmation.

`2` and `7` print “正在检测中” immediately. One check waits at most about 3 seconds and reports the latency. During a watch, press **Q** to return to the menu. A standalone `watch` command exits on Q.

```powershell
.\hust-wireless-rs.exe login
.\hust-wireless-rs.exe status
.\hust-wireless-rs.exe watch
.\hust-wireless-rs.exe install
.\hust-wireless-rs.exe uninstall
.\hust-wireless-rs.exe set-url
.\hust-wireless-rs.exe remove
```

`uninstall` only removes the startup entry. `remove` clears startup, the account, the log, and the saved login URL. It leaves the program files in place.

## When login fails

Choose `3` under more settings. A normal portal redirect carries fields such as `nasip` and `wlanuserip`. The diagnosis prints names, not values.

If it used to work and then stopped, the address has usually changed. Choose `9` and paste the current `index.jsp?` URL. A captcha or an unregistered-device message has to be handled in the browser before another login. The watch stops after five failed sign-ins in a row.

## Local data

```text
%APPDATA%\hust-wireless-cli\
├── config.json          # account, password stored with DPAPI
├── wired-login-url.txt  # login URL, plain text
└── hust-auth.log        # watch log
```

## Build from source

Rust stable is required.

```powershell
cd .\hust-wireless-rs
cargo build --release
```

Without the Visual Studio C++ linker:

```powershell
cargo +stable-x86_64-pc-windows-gnu build --release
```

The GNU target needs a 64-bit MinGW `bin` directory on `PATH`. Run `target\release\hust-wireless-rs.exe`, or double-click `hust.cmd`.

```powershell
cargo test
```

## Notes

- The campus portal is HTTP, and the RSA public key comes over the same path. That is not HTTPS.
- DPAPI protects the password on disk. Another program running as the same Windows user can still decrypt it. The login URL contains device and session parameters.
- Connectivity probes use Xiaomi and Huawei `generate_204` endpoints, plus the campus portal and `123.123.123.123`.
- Startup with Windows is available on Windows only.

## License

[MIT](LICENSE)
