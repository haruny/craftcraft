//! Finding apps that are already on this computer, however they got there: an installer, winget,
//! a package manager, cargo, or the user dragging an app to /Applications.
//!
//! Desktop apps are looked up where their installers register them (Windows' list of installed
//! programs, macOS app bundles, Linux packages). Command-line apps are looked up on PATH and asked
//! for `--version`. Apps the launcher unpacked into its own folder are skipped here; it already
//! knows about those.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::catalog::{App, Kind, Os, Platform};
use crate::paths::{self, Paths};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Detected {
    /// As the installer or the program reports it (`0.16.0`); `None` when it can't be told.
    pub version: Option<String>,
    /// The program to open, when found.
    pub exe: Option<PathBuf>,
    /// How to remove it, on Windows: the installer's uninstall command.
    pub uninstall: Option<String>,
    /// On Windows: installed for every user, so an update has to be too.
    pub for_everyone: bool,
}

pub fn detect_all(apps: &[App], platform: Platform, paths: &Paths) -> HashMap<String, Detected> {
    let registered = registered_programs();
    let path_dirs: Vec<PathBuf> = std::env::var_os("PATH")
        .map(|p| std::env::split_paths(&p).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        // Our own shims and unpacked apps are tracked separately.
        .filter(|d| !d.starts_with(&paths.root))
        .collect();
    apps.iter()
        .filter_map(|app| {
            let found = if app.kind == Kind::Gui {
                detect_gui(app, platform.os, &registered, &path_dirs)
            } else {
                detect_command(app, platform.os, &path_dirs).or_else(|| detect_gui(app, platform.os, &registered, &path_dirs))
            };
            found.map(|d| (app.id.clone(), d))
        })
        .collect()
}

fn bin_names(app: &App) -> Vec<String> {
    if app.bin.is_empty() { vec![app.id.clone()] } else { app.bin.clone() }
}

fn exe_name(bin: &str, os: Os) -> String {
    if os == Os::Windows && !bin.to_ascii_lowercase().ends_with(".exe") { format!("{bin}.exe") } else { bin.to_string() }
}

/// A command-line app on PATH, with the version it reports.
fn detect_command(app: &App, os: Os, path_dirs: &[PathBuf]) -> Option<Detected> {
    let exe = bin_names(app).iter().find_map(|bin| find_on_path(&exe_name(bin, os), path_dirs))?;
    let version = run_version(&exe);
    Some(Detected { version, exe: Some(exe), uninstall: None, for_everyone: false })
}

fn find_on_path(file: &str, dirs: &[PathBuf]) -> Option<PathBuf> {
    dirs.iter().map(|d| d.join(file)).find(|p| p.is_file())
}

fn detect_gui(app: &App, os: Os, registered: &[Registered], path_dirs: &[PathBuf]) -> Option<Detected> {
    match os {
        Os::Windows => detect_windows(app, registered),
        Os::Macos => detect_macos(app),
        Os::Linux => detect_linux(app, path_dirs),
    }
}

/// One entry in Windows' list of installed programs.
#[derive(Clone, Debug, Default)]
struct Registered {
    name: String,
    version: Option<String>,
    location: Option<PathBuf>,
    icon: Option<PathBuf>,
    uninstall: Option<String>,
    /// Installed for every user (see [`registered_programs`]).
    for_everyone: bool,
}

/// Whether an installed program's display name is this app: `PhotoCraft`, `PhotoCraft 0.16.0`
/// and `DesignCraft (Craft Apps)` are; `ArtCraft Launcher` isn't ArtCraft.
fn name_matches(display: &str, name: &str) -> bool {
    let display = display.trim();
    if display.len() < name.len() || !display.is_char_boundary(name.len()) || !display[..name.len()].eq_ignore_ascii_case(name) {
        return false;
    }
    let rest = &display[name.len()..];
    rest.is_empty()
        || rest.starts_with(" (")
        || rest.strip_prefix(' ').is_some_and(|r| r.trim_start_matches(['v', 'V']).starts_with(|c: char| c.is_ascii_digit()))
}

fn detect_windows(app: &App, registered: &[Registered]) -> Option<Detected> {
    let launch = app.launch_path(Os::Windows).map(paths::expand).filter(|p| p.is_file());
    let mut matches: Vec<&Registered> = registered.iter().filter(|r| name_matches(&r.name, &app.name)).collect();
    // The newest, if a program is listed twice.
    matches.sort_by(|a, b| {
        let newer = |x: &Registered, y: &Registered| crate::version::is_newer(x.version.as_deref().unwrap_or(""), y.version.as_deref().unwrap_or(""));
        if newer(a, b) {
            std::cmp::Ordering::Less
        } else if newer(b, a) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });
    let Some(entry) = matches.first() else {
        return launch.map(|exe| Detected { version: None, exe: Some(exe), uninstall: None, for_everyone: false });
    };
    let exe = launch.or_else(|| find_windows_exe(app, matches.as_slice()));
    Some(Detected { version: entry.version.clone(), exe, uninstall: matches.iter().find_map(|r| r.uninstall.clone()), for_everyone: entry.for_everyone })
}

/// Where an installed program's executable is: from its icon, its install folder, or the usual
/// Program Files folders.
fn find_windows_exe(app: &App, entries: &[&Registered]) -> Option<PathBuf> {
    let files: Vec<String> = bin_names(app).iter().map(|b| exe_name(b, Os::Windows)).collect();
    let is_ours = |p: &Path| p.file_name().is_some_and(|f| files.iter().any(|n| f.to_string_lossy().eq_ignore_ascii_case(n)));
    let mut dirs = Vec::new();
    for e in entries {
        if let Some(icon) = &e.icon {
            if is_ours(icon) && icon.is_file() {
                return Some(icon.clone());
            }
            dirs.extend(icon.parent().map(Path::to_path_buf));
        }
        dirs.extend(e.location.clone());
    }
    for var in ["ProgramFiles", "ProgramW6432", "LOCALAPPDATA"] {
        if let Ok(base) = std::env::var(var) {
            let base = PathBuf::from(base);
            dirs.push(if var == "LOCALAPPDATA" { base.join("Programs").join(&app.name) } else { base.join(&app.name) });
        }
    }
    dirs.iter().filter(|d| d.is_dir()).find_map(|d| files.iter().find_map(|f| find_shallow(d, f, 2)))
}

fn find_shallow(dir: &Path, file: &str, depth: usize) -> Option<PathBuf> {
    let candidate = dir.join(file);
    if candidate.is_file() {
        return Some(candidate);
    }
    if depth == 0 {
        return None;
    }
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
        .find_map(|e| find_shallow(&e.path(), file, depth - 1))
}

#[cfg(windows)]
fn registered_programs() -> Vec<Registered> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};

    const UNINSTALL: &str = r"Software\Microsoft\Windows\CurrentVersion\Uninstall";
    const UNINSTALL_32: &str = r"Software\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall";
    let mut out = Vec::new();
    for (root, path) in [(HKEY_LOCAL_MACHINE, UNINSTALL), (HKEY_LOCAL_MACHINE, UNINSTALL_32), (HKEY_CURRENT_USER, UNINSTALL)] {
        let Ok(key) = RegKey::predef(root).open_subkey(path) else { continue };
        for name in key.enum_keys().filter_map(Result::ok) {
            let Ok(sub) = key.open_subkey(&name) else { continue };
            let get = |v: &str| sub.get_value::<String, _>(v).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
            let Some(display) = get("DisplayName") else { continue };
            // Windows updates and other system components aren't apps.
            if sub.get_value::<u32, _>("SystemComponent").is_ok_and(|v| v == 1) {
                continue;
            }
            let uninstall = get("UninstallString");
            // Other installers list a copy for everyone in HKLM and one for a user in HKCU, but
            // Windows Installer lists both in HKLM: only its own records tell them apart.
            let windows_installer = sub.get_value::<u32, _>("WindowsInstaller").is_ok_and(|v| v == 1);
            let for_everyone = match msi_product(&name, windows_installer, uninstall.as_deref()) {
                Some(product) => msi_for_everyone(&product),
                None => root == HKEY_LOCAL_MACHINE,
            };
            out.push(Registered {
                name: display,
                version: get("DisplayVersion"),
                location: get("InstallLocation").map(PathBuf::from),
                icon: get("DisplayIcon").map(|i| PathBuf::from(i.split(',').next().unwrap_or_default().trim_matches('"'))),
                uninstall,
                for_everyone,
            });
        }
    }
    out
}

#[cfg(not(windows))]
fn registered_programs() -> Vec<Registered> {
    Vec::new()
}

/// Whether Windows Installer installed `product` for every user: then it's listed among the
/// computer's products, not only a user's.
#[cfg(windows)]
fn msi_for_everyone(product: &str) -> bool {
    use winreg::RegKey;
    use winreg::enums::HKEY_LOCAL_MACHINE;
    packed_code(product).is_some_and(|packed| RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(format!(r"SOFTWARE\Classes\Installer\Products\{packed}")).is_ok())
}

/// The Windows Installer product behind an Uninstall entry: the entry's own name when Windows
/// Installer wrote it, or the product its `msiexec /x {…}` removes.
#[cfg_attr(not(windows), allow(dead_code))]
fn msi_product(key: &str, windows_installer: bool, uninstall: Option<&str>) -> Option<String> {
    if windows_installer && is_guid(key) {
        return Some(key.to_string());
    }
    let command = uninstall.filter(|u| u.to_ascii_lowercase().contains("msiexec"))?;
    let start = command.find('{')?;
    command.get(start..start + 38).filter(|code| is_guid(code)).map(str::to_string)
}

/// `{1C6822F4-52C8-47B6-B76D-C410EA5220CF}`
#[cfg_attr(not(windows), allow(dead_code))]
fn is_guid(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 38
        && b[0] == b'{'
        && b[37] == b'}'
        && b[1..37].iter().enumerate().all(|(i, c)| if [8, 13, 18, 23].contains(&i) { *c == b'-' } else { c.is_ascii_hexdigit() })
}

/// How Windows Installer's registry spells a product code: the first three groups reversed, then
/// each pair of digits after them swapped.
#[cfg_attr(not(windows), allow(dead_code))]
fn packed_code(code: &str) -> Option<String> {
    if !is_guid(code) {
        return None;
    }
    let hex: Vec<char> = code.chars().filter(char::is_ascii_hexdigit).map(|c| c.to_ascii_uppercase()).collect();
    let mut out: String = [0..8, 8..12, 12..16].into_iter().flat_map(|r| hex[r].iter().rev()).collect();
    for pair in hex[16..].chunks(2) {
        out.extend([pair[1], pair[0]]);
    }
    Some(out)
}

fn detect_macos(app: &App) -> Option<Detected> {
    let mut bundles: Vec<PathBuf> = vec![PathBuf::from(format!("/Applications/{}.app", app.name))];
    if let Some(home) = directories::BaseDirs::new() {
        bundles.push(home.home_dir().join("Applications").join(format!("{}.app", app.name)));
    }
    bundles.extend(app.launch_path(Os::Macos).map(paths::expand));
    let bundle = bundles.into_iter().find(|b| b.is_dir())?;
    let version = std::fs::read_to_string(bundle.join("Contents/Info.plist")).ok().and_then(|plist| plist_string(&plist, "CFBundleShortVersionString"));
    Some(Detected { version, exe: Some(bundle), uninstall: None, for_everyone: false })
}

/// A string value from an XML property list.
fn plist_string(plist: &str, key: &str) -> Option<String> {
    let after = &plist[plist.find(&format!("<key>{key}</key>"))? + key.len() + 11..];
    let start = after.find("<string>")? + 8;
    let end = after[start..].find("</string>")?;
    Some(after[start..start + end].trim().to_string()).filter(|v| !v.is_empty())
}

fn detect_linux(app: &App, path_dirs: &[PathBuf]) -> Option<Detected> {
    let launch = app.launch_path(Os::Linux).map(paths::expand).filter(|p| p.is_file());
    if let Some(exe) = launch.or_else(|| bin_names(app).iter().find_map(|b| find_on_path(b, path_dirs))) {
        return Some(Detected { version: package_version(&app.id), exe: Some(exe), uninstall: None, for_everyone: false });
    }
    // An AppImage the user keeps in ~/Applications, named like `photocraft-0.16.0-linux-x86_64.AppImage`.
    let dir = directories::BaseDirs::new()?.home_dir().join("Applications");
    let prefix = format!("{}-", app.id);
    let image = std::fs::read_dir(dir).ok()?.filter_map(Result::ok).map(|e| e.path()).find(|p| {
        let name = p.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
        name.starts_with(&prefix) && name.ends_with(".appimage")
    })?;
    let name = image.file_name()?.to_string_lossy().into_owned();
    Some(Detected { version: parse_version(&name[prefix.len()..]), exe: Some(image), uninstall: None, for_everyone: false })
}

/// The version of an installed .deb or .rpm package.
fn package_version(name: &str) -> Option<String> {
    let tries: [(&str, &[&str]); 2] = [("dpkg-query", &["-W", "-f=${Version}"]), ("rpm", &["-q", "--qf", "%{VERSION}"])];
    tries.iter().find_map(|(cmd, args)| {
        let out = Command::new(cmd).args(*args).arg(name).stderr(Stdio::null()).output().ok()?;
        out.status.success().then(|| parse_version(&String::from_utf8_lossy(&out.stdout))).flatten()
    })
}

/// Runs `exe --version` (hidden, for at most three seconds) and reads the version from it.
fn run_version(exe: &Path) -> Option<String> {
    let mut cmd = Command::new(exe);
    cmd.arg("--version").stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let mut child = cmd.spawn().ok()?;
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(30)),
            _ => {
                _ = child.kill();
                _ = child.wait();
                return None;
            }
        }
    }
    let mut out = String::new();
    std::io::Read::read_to_string(&mut child.stdout.take()?, &mut out).ok()?;
    parse_version(&out)
}

/// The first dotted version number in `text`: `ripgrep 15.2.0 (rev abc)` → `15.2.0`.
fn parse_version(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let boundary = i == 0 || !bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'v';
        if bytes[i].is_ascii_digit() && boundary {
            let end = text[i..].find(|c: char| !(c.is_ascii_digit() || c == '.')).map_or(text.len(), |n| i + n);
            let candidate = text[i..end].trim_end_matches('.');
            if candidate.contains('.') {
                return Some(candidate.to_string());
            }
            i = end;
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_names_strictly() {
        assert!(name_matches("PhotoCraft", "PhotoCraft"));
        assert!(name_matches("DesignCraft (Craft Apps)", "DesignCraft"));
        assert!(name_matches("Zed 1.25.3", "Zed"));
        assert!(name_matches("photocraft", "PhotoCraft"));
        assert!(!name_matches("ArtCraft Launcher", "ArtCraft"));
        assert!(!name_matches("Leathercraft CAD version 3.5.0", "CADCraft"));
        assert!(!name_matches("Zedd", "Zed"));
        assert!(!name_matches("Ze", "Zed"));
    }

    #[test]
    fn reads_versions() {
        assert_eq!(parse_version("ripgrep 15.2.0 (rev 1a2b3c)\n").as_deref(), Some("15.2.0"));
        assert_eq!(parse_version("uv 0.13.5 (abc 2026-10-01)").as_deref(), Some("0.13.5"));
        assert_eq!(parse_version("nu v0.116.1").as_deref(), Some("0.116.1"));
        assert_eq!(parse_version("x86_64 build 7").as_deref(), None);
        assert_eq!(parse_version("0.16.0-linux-x86_64.AppImage").as_deref(), Some("0.16.0"));
    }

    #[test]
    fn reads_plists() {
        let plist = "<dict>\n<key>CFBundleName</key>\n<string>PhotoCraft</string>\n<key>CFBundleShortVersionString</key>\n\t<string>0.16.0</string>\n</dict>";
        assert_eq!(plist_string(plist, "CFBundleShortVersionString").as_deref(), Some("0.16.0"));
        assert_eq!(plist_string(plist, "Missing"), None);
    }

    #[test]
    fn picks_the_newest_registry_entry() {
        let text = r#"{"schema":1,"name":"t","launcher":{"version":"0","repo":"o/l"},"apps":[{"id":"designcraft","name":"DesignCraft",
            "replaces":["InDesign"],"category":"c","kind":"gui","tagline":"t","repo":"o/designcraft","assets":{}}]}"#;
        let app = crate::catalog::Catalog::parse(text).unwrap().catalog.apps.remove(0);
        let reg = |name: &str, version: &str, uninstall: Option<&str>| Registered {
            name: name.into(),
            version: Some(version.into()),
            uninstall: uninstall.map(Into::into),
            ..Default::default()
        };
        let registered = [
            reg("DesignCraft (Craft Apps)", "0.12.0", None),
            Registered { for_everyone: true, ..reg("DesignCraft", "0.13.0", Some("MsiExec.exe /X{60C26D84}")) },
            reg("ArtCraft Launcher", "9.9.9", None),
        ];
        let found = detect_windows(&app, &registered).unwrap();
        assert_eq!(found.version.as_deref(), Some("0.13.0"));
        assert_eq!(found.uninstall.as_deref(), Some("MsiExec.exe /X{60C26D84}"));
        assert!(found.for_everyone);
    }

    #[test]
    fn finds_windows_installer_products() {
        // CraftCraft installed just for one user, and DeckCraft for everyone, as listed on a real computer.
        assert_eq!(packed_code("{1C6822F4-52C8-47B6-B76D-C410EA5220CF}").as_deref(), Some("4F2286C18C256B747BD64C01AE2502FC"));
        assert_eq!(packed_code("{6d0c3f9e-8e6f-4669-9822-0711634ce881}").as_deref(), Some("E9F3C0D6F6E896648922701136C48E18"));
        assert_eq!(packed_code("{6D0C3F9E-8E6F-4669-9822-0711634CE88}"), None);

        let code = "{6D0C3F9E-8E6F-4669-9822-0711634CE881}";
        assert_eq!(msi_product(code, true, Some(&format!("MsiExec.exe /I{code}"))).as_deref(), Some(code));
        // An installer's own entry, removing the product it installed.
        let designcraft = r#""C:\Windows\system32\msiexec.exe" /x {449AFDF4-C57B-4451-8602-B5CA5A0E3D38} /qf /norestart"#;
        assert_eq!(msi_product("Craft.designcraft", false, Some(designcraft)).as_deref(), Some("{449AFDF4-C57B-4451-8602-B5CA5A0E3D38}"));
        // Inno Setup's `{…}_is1` entries aren't Windows Installer's.
        assert_eq!(msi_product("{444B39B7-B4F5-4591-8196-5A4CC453D928}_is1", false, Some(r#""C:\x\unins000.exe""#)), None);
        assert_eq!(msi_product(code, false, None), None);
    }
}
