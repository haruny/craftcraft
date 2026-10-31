//! Opening things: apps, terminals, folders, links, and the user's PATH.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::catalog::{App, Kind, Os, Platform};
use crate::detect::Detected;
use crate::paths::Paths;
use crate::state::Installed;

/// Opens an app: a window for desktop apps, a terminal for the rest. `installed` is a copy the
/// launcher unpacked; `detected` one found on the computer.
pub fn open_app(app: &App, installed: Option<&Installed>, detected: Option<&Detected>, paths: &Paths, platform: Platform) -> Result<(), String> {
    let bin: Option<PathBuf> = installed.and_then(|i| i.bins.first().cloned()).or_else(|| detected.and_then(|d| d.exe.clone()));
    if app.kind == Kind::Gui {
        let target = bin.ok_or("couldn't find where the app is installed")?;
        return open_gui(&target, platform.os);
    }
    let name = bin
        .as_deref()
        .and_then(Path::file_stem)
        .map(|s| s.to_string_lossy().into_owned())
        .or_else(|| app.bin.first().cloned())
        .ok_or("couldn't find the installed command")?;
    let command = if app.kind == Kind::Tui { Some(name.as_str()) } else { None };
    open_terminal(&paths.bin(), command, &name, platform.os)
}

fn open_gui(target: &Path, os: Os) -> Result<(), String> {
    // A macOS bundle opens through Launch Services, which brings it to the front properly.
    if os == Os::Macos
        && let Some(bundle) = target.ancestors().find(|p| p.extension().is_some_and(|e| e == "app"))
    {
        return spawn(Command::new("open").arg(bundle));
    }
    let mut cmd = Command::new(target);
    if let Some(dir) = target.parent() {
        cmd.current_dir(dir);
    }
    spawn(&mut cmd)
}

/// Opens a terminal with the launcher's `bin` folder on its PATH, running `command` if given and
/// otherwise saying how to use `name`.
fn open_terminal(bin_dir: &Path, command: Option<&str>, name: &str, os: Os) -> Result<(), String> {
    let bin = bin_dir.to_string_lossy();
    match os {
        Os::Windows => {
            let quoted = bin.replace('\'', "''");
            let run = command.map_or_else(|| format!("Write-Host '{name} is ready. Try: {name} --help'"), |c| format!("& '{c}'"));
            let script = format!("$env:PATH = '{quoted};' + $env:PATH; {run}");
            let mut cmd = Command::new("powershell.exe");
            cmd.args(["-NoExit", "-NoLogo", "-Command", &script]);
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                const CREATE_NEW_CONSOLE: u32 = 0x0000_0010;
                cmd.creation_flags(CREATE_NEW_CONSOLE);
            }
            if let Some(home) = directories::BaseDirs::new() {
                cmd.current_dir(home.home_dir());
            }
            spawn(&mut cmd)
        }
        Os::Macos | Os::Linux => {
            let quoted = bin.replace('\'', r"'\''");
            let run = command.map_or_else(|| format!("echo '{name} is ready. Try: {name} --help'"), |c| format!("'{c}'"));
            let script = format!("cd; export PATH='{quoted}':\"$PATH\"; clear; {run}");
            if os == Os::Macos {
                let apple = script.replace('\\', "\\\\").replace('"', "\\\"");
                spawn(Command::new("osascript").args([
                    "-e",
                    &format!("tell application \"Terminal\" to do script \"{apple}\""),
                    "-e",
                    "tell application \"Terminal\" to activate",
                ]))
            } else {
                let shell = format!("{script}; exec \"${{SHELL:-sh}}\"");
                let terminals: [(&str, &[&str]); 7] = [
                    ("x-terminal-emulator", &["-e"]),
                    ("gnome-terminal", &["--"]),
                    ("konsole", &["-e"]),
                    ("xfce4-terminal", &["-x"]),
                    ("kitty", &[]),
                    ("alacritty", &["-e"]),
                    ("xterm", &["-e"]),
                ];
                for (term, args) in terminals {
                    if Command::new(term).args(args).args(["sh", "-c", &shell]).spawn().is_ok() {
                        return Ok(());
                    }
                }
                Err("couldn't find a terminal to open".into())
            }
        }
    }
}

fn spawn(cmd: &mut Command) -> Result<(), String> {
    cmd.spawn().map(drop).map_err(|e| format!("couldn't start it: {e}"))
}

/// Shows a folder (or the folder holding a file) in the file manager.
pub fn reveal(path: &Path) {
    let dir = if path.is_dir() { path } else { path.parent().unwrap_or(path) };
    if let Err(e) = open::that_detached(dir) {
        eprintln!("opening {}: {e}", dir.display());
    }
}

/// Opens a file (a log, say) in the app the system uses for it.
pub fn open_file(path: &Path) {
    if let Err(e) = open::that_detached(path) {
        eprintln!("opening {}: {e}", path.display());
    }
}

/// The commands with a launcher in the `bin` folder, by name.
pub fn commands(paths: &Paths) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(paths.bin()) else { return Vec::new() };
    let mut names: Vec<String> = entries.filter_map(Result::ok).filter_map(|e| Some(e.path().file_stem()?.to_string_lossy().into_owned())).collect();
    names.sort();
    names
}

/// Opens a web link from the catalog in the browser, if it's a plain https link.
pub fn open_url(url: &str) {
    let safe = url.starts_with("https://") && !url.chars().any(|c| c.is_control() || c.is_whitespace() || "\"'`|&;<>^%$!(){}[]".contains(c));
    if !safe {
        eprintln!("not opening {url:?}");
        return;
    }
    if let Err(e) = open::that_detached(url) {
        eprintln!("opening {url}: {e}");
    }
}

/// Opens a URL the launcher built itself, whose query values are percent-encoded, so it may hold
/// the `&` and `%` that [`open_url`] turns away from catalog URLs.
pub fn open_own_url(url: &str) {
    if let Err(e) = open::that_detached(url) {
        eprintln!("opening {url}: {e}");
    }
}

/// Runs an installed program's own uninstaller (from Windows' list of installed programs) and
/// waits for it. Windows Installer products are removed with `msiexec /x {product}`.
pub fn run_uninstaller(command: &str) -> Result<(), String> {
    let lower = command.to_ascii_lowercase();
    let status = if lower.contains("msiexec") {
        let start = command.find('{').ok_or("unexpected uninstall command")?;
        let end = command[start..].find('}').ok_or("unexpected uninstall command")? + start;
        Command::new("msiexec").arg("/x").arg(&command[start..=end]).status()
    } else {
        let (exe, args) = match command.strip_prefix('"') {
            Some(rest) => rest.split_once('"').map(|(e, a)| (e.to_string(), a.trim().to_string())).ok_or("unexpected uninstall command")?,
            None => command.split_once(' ').map_or((command.to_string(), String::new()), |(e, a)| (e.to_string(), a.to_string())),
        };
        Command::new(exe).args(args.split_whitespace()).status()
    }
    .map_err(|e| format!("couldn't start the uninstaller: {e}"))?;
    match status.code() {
        Some(0 | 3010) => Ok(()),
        Some(1602) => Err("the uninstall was cancelled".into()),
        Some(code) => Err(format!("the uninstaller stopped with code {code}")),
        None => Err("the uninstaller was stopped".into()),
    }
}

/// Opens the system's own app list, to remove an app a system installer put there.
pub fn open_system_uninstall(os: Os) {
    let target = match os {
        Os::Windows => "ms-settings:appsfeatures",
        Os::Macos => "/Applications",
        Os::Linux => return,
    };
    _ = open::that_detached(target);
}

/// Whether the launcher's `bin` folder is on the user's PATH.
pub fn bin_on_path(paths: &Paths) -> bool {
    let bin = paths.bin();
    #[cfg(windows)]
    if let Some(path) = windows_user_path() {
        return std::env::split_paths(&path).any(|p| p == bin);
    }
    std::env::var_os("PATH").is_some_and(|p| std::env::split_paths(&p).any(|p| p == bin))
}

#[cfg(windows)]
fn windows_user_path() -> Option<String> {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;
    RegKey::predef(HKEY_CURRENT_USER).open_subkey("Environment").ok()?.get_value("Path").ok()
}

/// Adds the `bin` folder to the user's PATH (Windows). New terminals see it straight away.
#[cfg(windows)]
pub fn add_bin_to_path(paths: &Paths) -> Result<(), String> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE, RegType};
    use winreg::{RegKey, RegValue};

    let env = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE).map_err(|e| e.to_string())?;
    let current: String = env.get_value("Path").unwrap_or_default();
    let bin = paths.bin().to_string_lossy().into_owned();
    if current.split(';').any(|p| p.eq_ignore_ascii_case(&bin)) {
        return Ok(());
    }
    let updated = if current.is_empty() || current.ends_with(';') { format!("{current}{bin}") } else { format!("{current};{bin}") };
    // REG_EXPAND_SZ keeps any %VARIABLES% already in the user's PATH working.
    let mut bytes: Vec<u8> = updated.encode_utf16().chain(std::iter::once(0)).flat_map(u16::to_le_bytes).collect();
    bytes.shrink_to_fit();
    env.set_raw_value("Path", &RegValue { bytes: bytes.into(), vtype: RegType::REG_EXPAND_SZ }).map_err(|e| e.to_string())?;
    broadcast_environment_change();
    Ok(())
}

#[cfg(windows)]
fn broadcast_environment_change() {
    use windows_sys::Win32::UI::WindowsAndMessaging::{HWND_BROADCAST, SMTO_ABORTIFHUNG, SendMessageTimeoutW, WM_SETTINGCHANGE};
    let name: Vec<u16> = "Environment".encode_utf16().chain(std::iter::once(0)).collect();
    let mut result = 0;
    // SAFETY: `name` is a NUL-terminated UTF-16 string that outlives the call.
    unsafe {
        SendMessageTimeoutW(HWND_BROADCAST, WM_SETTINGCHANGE, 0, name.as_ptr() as isize, SMTO_ABORTIFHUNG, 3000, &mut result);
    }
}

#[cfg(windows)]
const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

/// Whether Windows starts the launcher when the user signs in.
#[cfg(windows)]
pub fn starts_on_logon() -> bool {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;
    RegKey::predef(HKEY_CURRENT_USER).open_subkey(RUN_KEY).and_then(|k| k.get_value::<String, _>("CraftCraft")).is_ok()
}

/// Has Windows start this copy of the launcher, in the tray, when the user signs in (or not).
#[cfg(windows)]
pub fn set_start_on_logon(on: bool) -> Result<(), String> {
    use winreg::RegKey;
    use winreg::enums::HKEY_CURRENT_USER;
    let (run, _) = RegKey::predef(HKEY_CURRENT_USER).create_subkey(RUN_KEY).map_err(|e| e.to_string())?;
    if on {
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        run.set_value("CraftCraft", &format!("\"{}\" --background", exe.display())).map_err(|e| e.to_string())
    } else {
        match run.delete_value("CraftCraft") {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.to_string()),
            _ => Ok(()),
        }
    }
}

/// The LaunchAgent that starts the launcher at login (macOS).
#[cfg(target_os = "macos")]
fn launch_agent() -> Option<PathBuf> {
    Some(directories::BaseDirs::new()?.home_dir().join("Library/LaunchAgents/io.craftcraft.CraftCraft.plist"))
}

/// Whether macOS starts the launcher when the user logs in.
#[cfg(target_os = "macos")]
pub fn starts_on_logon() -> bool {
    launch_agent().is_some_and(|p| p.is_file())
}

/// Has macOS start this copy of the launcher, in the menu bar, when the user logs in (or not).
#[cfg(target_os = "macos")]
pub fn set_start_on_logon(on: bool) -> Result<(), String> {
    let agent = launch_agent().ok_or("couldn't find your home folder")?;
    if !on {
        return match std::fs::remove_file(&agent) {
            Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.to_string()),
            _ => Ok(()),
        };
    }
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe = exe.to_string_lossy().replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>io.craftcraft.CraftCraft</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exe}</string>
        <string>--background</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>ProcessType</key>
    <string>Interactive</string>
</dict>
</plist>
"#
    );
    if let Some(dir) = agent.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    crate::paths::write_atomic(&agent, plist.as_bytes()).map_err(|e| e.to_string())
}

/// Adds the launcher to the Start menu if it isn't there, as the install script does, for a copy
/// installed some other way (unpacked from the zip, say). Debug builds leave it alone, and so does
/// a copy the installer put in everyone's Start menu.
#[cfg(windows)]
pub fn add_to_start_menu() {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    let (Some(dirs), Ok(exe)) = (directories::BaseDirs::new(), std::env::current_exe()) else { return };
    let programs = r"Microsoft\Windows\Start Menu\Programs\CraftCraft.lnk";
    let link = dirs.data_dir().join(programs);
    let everyones = std::env::var_os("ProgramData").map(|d| PathBuf::from(d).join(programs));
    if cfg!(debug_assertions) || link.exists() || everyones.is_some_and(|l| l.exists()) {
        return;
    }
    let quote = |p: &Path| p.to_string_lossy().replace('\'', "''");
    let dir = exe.parent().unwrap_or(&exe);
    let script = format!(
        "$l = (New-Object -ComObject WScript.Shell).CreateShortcut('{}'); $l.TargetPath = '{exe}'; $l.WorkingDirectory = '{}'; \
         $l.IconLocation = '{exe},0'; $l.Description = 'Install, update and open software rewritten in Rust'; $l.Save()",
        quote(&link),
        quote(dir),
        exe = quote(&exe),
    );
    let spawned = Command::new("powershell.exe").args(["-NoProfile", "-NonInteractive", "-Command", &script]).creation_flags(CREATE_NO_WINDOW).spawn();
    if let Err(e) = spawned {
        eprintln!("adding to the Start menu: {e}");
    }
}

/// The line to add to a shell profile (macOS and Linux).
#[cfg(not(windows))]
pub fn path_line(paths: &Paths) -> String {
    format!("export PATH=\"{}:$PATH\"", paths.bin().display())
}
