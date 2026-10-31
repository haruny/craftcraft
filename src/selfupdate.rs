//! Updating the launcher itself, from its own GitHub releases.
//!
//! The new version is downloaded in the background, checked against the SHA-256 GitHub publishes
//! (a download without one is refused), and unpacked; it takes over when the launcher restarts.
//! How depends on how this copy was installed:
//!
//! - Windows, by the .msi: the new .msi, run by a small PowerShell script that outlives the
//!   launcher (the installer stops it), then starts the new version.
//! - Anywhere else the launcher can write to its own folder (install.ps1, install.sh, the zip,
//!   the disk image dragged to Applications): the new executable, or on macOS the new bundle, is
//!   renamed into place, which works while the old one runs, and started.
//! - Otherwise (a folder it can't write to, say from a Linux package): the user downloads it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::github::{Lookup, Releases};
use crate::http::Http;
use crate::paths::Paths;
use crate::{debug, version};

/// Windows Installer's log of the update, in the logs folder.
#[cfg_attr(not(windows), allow(dead_code))]
const MSI_LOG: &str = "craftcraft-update.log";

/// How this copy of the launcher can be updated.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum How {
    /// Windows Installer put it there; for everyone, Windows asks for an administrator's approval.
    Msi { for_everyone: bool },
    /// By renaming the new copy over `target`: the executable, or on macOS the .app bundle.
    Replace { target: PathBuf },
    /// Only by hand, for this reason.
    Manual(String),
}

/// A new version, downloaded and checked, waiting for the launcher to restart into it.
#[derive(Clone, Debug)]
pub struct Ready {
    /// `0.2.0`, for display.
    pub version: String,
    how: How,
    /// The downloaded .msi, or the unpacked executable or bundle.
    staged: PathBuf,
}

impl Ready {
    /// Whether restarting into it asks for an administrator's approval, so it shouldn't happen
    /// unprompted.
    pub fn asks_for_approval(&self) -> bool {
        self.how == How::Msi { for_everyone: true }
    }
}

/// How this copy can be updated.
pub fn how() -> How {
    if cfg!(debug_assertions) {
        return How::Manual("this is a development build".into());
    }
    let Ok(exe) = std::env::current_exe() else { return How::Manual("couldn't find the launcher's own file".into()) };
    #[cfg(windows)]
    if let Some(for_everyone) = msi_scope(&exe) {
        return How::Msi { for_everyone };
    }
    // A macOS bundle is replaced whole.
    let target = if cfg!(target_os = "macos") {
        exe.ancestors().find(|p| p.extension().is_some_and(|e| e == "app")).map_or_else(|| exe.clone(), Path::to_path_buf)
    } else {
        exe
    };
    let Some(dir) = target.parent() else { return How::Manual("couldn't find the launcher's folder".into()) };
    if !writable(dir) {
        return How::Manual(format!("CraftCraft can't write to {}", dir.display()));
    }
    How::Replace { target }
}

/// Whether the launcher may create files in `dir`.
fn writable(dir: &Path) -> bool {
    let probe = dir.join(format!(".craftcraft-{}.probe", std::process::id()));
    let ok = fs::File::create(&probe).is_ok();
    _ = fs::remove_file(&probe);
    ok
}

/// Whether the .msi installed this copy, and if so whether for everyone. Its Start menu shortcut
/// leaves a mark in HKLM (everyone) or HKCU (just this user), and it installs in Program Files or
/// in the same folder as install.ps1, whose copy has no mark.
#[cfg(windows)]
fn msi_scope(exe: &Path) -> Option<bool> {
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    let marked = |root| RegKey::predef(root).open_subkey(r"Software\CraftCraft\Installer").and_then(|k| k.get_value::<u32, _>("StartMenu")).is_ok();
    let dir = exe.parent()?.to_string_lossy().to_lowercase();
    let under = |var: &str, sub: &str| std::env::var_os(var).is_some_and(|d| dir == PathBuf::from(d).join(sub).to_string_lossy().to_lowercase());
    if under("ProgramFiles", "CraftCraft") && marked(HKEY_LOCAL_MACHINE) {
        return Some(true);
    }
    if under("LOCALAPPDATA", r"Programs\CraftCraft") && marked(HKEY_CURRENT_USER) {
        return Some(false);
    }
    None
}

/// The release file this copy updates from.
fn asset_name(how: &How) -> Option<String> {
    let arch = std::env::consts::ARCH;
    if cfg!(windows) {
        Some(format!("craftcraft-windows-{arch}.{}", if matches!(how, How::Msi { .. }) { "msi" } else { "zip" }))
    } else if cfg!(target_os = "macos") {
        Some("craftcraft-macos-universal.tar.gz".into())
    } else if cfg!(target_os = "linux") {
        Some(format!("craftcraft-linux-{arch}.tar.gz"))
    } else {
        None
    }
}

/// Where a new version waits.
fn update_dir(paths: &Paths) -> PathBuf {
    paths.root.join("update")
}

/// A folder to update from instead of GitHub, holding the release's files and `SHA256SUMS`, as
/// `CRAFTCRAFT_DIST` is for the install scripts. For testing.
fn test_dist() -> Option<PathBuf> {
    std::env::var_os("CRAFTCRAFT_UPDATE_DIST").filter(|d| !d.is_empty()).map(PathBuf::from)
}

/// Downloads, checks and unpacks the latest release of `repo`, if it's newer than this launcher.
/// From a test folder (see [`test_dist`]), the files there are taken to be version `offered`.
pub fn prepare(http: &Http, paths: &Paths, releases: &Releases, repo: &str, offered: &str, how: &How) -> Result<Ready, String> {
    if let How::Manual(why) = how {
        return Err(why.clone());
    }
    let name = asset_name(how).ok_or("there's no CraftCraft build for this computer")?;
    let (tag, source, expected) = match test_dist() {
        Some(dist) => {
            let sums = fs::read_to_string(dist.join("SHA256SUMS")).map_err(|e| format!("{}: {e}", dist.join("SHA256SUMS").display()))?;
            let expected = sums
                .lines()
                .find_map(|l| l.split_once(char::is_whitespace).filter(|(_, f)| f.trim().trim_start_matches('*') == name).map(|(h, _)| h.to_ascii_lowercase()))
                .ok_or_else(|| format!("SHA256SUMS in {} has no {name}", dist.display()))?;
            (offered.to_string(), Err(dist.join(&name)), expected)
        }
        None => {
            let release = match releases.latest(repo) {
                Lookup::Found(r) => r,
                Lookup::NoRelease => return Err("CraftCraft has no release yet".into()),
                Lookup::Failed(e) => return Err(e),
            };
            let asset = release.assets.iter().find(|a| a.name == name).ok_or_else(|| format!("release {} has no {name}", release.tag))?;
            let expected = asset.sha256.clone().ok_or_else(|| format!("{name} has no published SHA-256, so it wasn't used"))?;
            (release.tag.clone(), Ok(asset.url.clone()), expected)
        }
    };
    let current = env!("CARGO_PKG_VERSION");
    if !version::is_newer(&tag, current) {
        return Err(format!("the latest release, {tag}, isn't newer than {current} yet"));
    }
    let new_version = version::display(&tag).to_string();

    let dir = update_dir(paths);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| format!("couldn't clear {}: {e}", dir.display()))?;
    }
    fs::create_dir_all(&dir).map_err(|e| format!("couldn't create {}: {e}", dir.display()))?;
    let file = dir.join(&name);
    let sha = match &source {
        Ok(url) => {
            debug::log(format_args!("CraftCraft {new_version}: downloading {url}"));
            http.download(url, &file, |_, _| {})?
        }
        Err(local) => {
            debug::log(format_args!("CraftCraft {new_version}: copying {}", local.display()));
            fs::copy(local, &file).map_err(|e| format!("{}: {e}", local.display()))?;
            let bytes = fs::read(&file).map_err(|e| e.to_string())?;
            crate::http::to_hex(&<sha2::Sha256 as sha2::Digest>::digest(&bytes))
        }
    };
    if sha != expected {
        _ = fs::remove_file(&file);
        return Err(format!("{name} failed its checksum (expected {expected}, got {sha}); it was deleted"));
    }
    let staged = if matches!(how, How::Msi { .. }) {
        file
    } else {
        let unpacked = dir.join("unpacked");
        crate::install::extract(&file, &name, &unpacked)?;
        _ = fs::remove_file(&file);
        let inside = if cfg!(windows) {
            "craftcraft.exe"
        } else if cfg!(target_os = "macos") {
            "CraftCraft.app"
        } else {
            "craftcraft"
        };
        let staged = unpacked.join(inside);
        if !staged.exists() {
            return Err(format!("{name} has no {inside}"));
        }
        // Windows builds are windowed apps, whose --version can't be read reliably.
        #[cfg(unix)]
        check_runs(&staged, &new_version)?;
        staged
    };
    debug::log(format_args!("CraftCraft {new_version} is ready at {}", staged.display()));
    Ok(Ready { version: new_version, how: how.clone(), staged })
}

/// Runs the new version's `--version`, which also shows it runs on this computer.
#[cfg(unix)]
fn check_runs(staged: &Path, expected: &str) -> Result<(), String> {
    let exe = if cfg!(target_os = "macos") { staged.join("Contents/MacOS/craftcraft") } else { staged.to_path_buf() };
    let out = Command::new(&exe).arg("--version").output().map_err(|e| format!("the new version doesn't run: {e}"))?;
    let said = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if said == format!("craftcraft {expected}") { Ok(()) } else { Err(format!("the new version says {said:?}, not craftcraft {expected}")) }
}

/// Puts the new version in place and starts it, in the tray if `background`. The caller then
/// quits, which on Windows the new version waits for.
pub fn apply(ready: &Ready, paths: &Paths, background: bool) -> Result<(), String> {
    let mut args = vec!["--restarted"];
    if background {
        args.push("--background");
    }
    debug::log(format_args!("restarting into CraftCraft {}", ready.version));
    match &ready.how {
        How::Manual(why) => Err(why.clone()),
        #[cfg(windows)]
        How::Msi { for_everyone } => run_msi(&ready.staged, paths, *for_everyone, &args),
        #[cfg(not(windows))]
        How::Msi { .. } => Err("not on Windows".into()),
        How::Replace { target } => {
            let _ = paths;
            replace(&ready.staged, target)?;
            #[cfg(windows)]
            set_listed_version(target, &ready.version);
            start(target, &args)
        }
    }
}

/// `craftcraft.exe` → `craftcraft.exe.old`.
fn sibling(path: &Path, suffix: &str) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(suffix);
    path.with_file_name(name)
}

fn remove_any(path: &Path) {
    _ = if path.is_dir() { fs::remove_dir_all(path) } else { fs::remove_file(path) };
}

/// Swaps `staged` in for `target` with two renames, putting the old one back if the second fails.
fn replace(staged: &Path, target: &Path) -> Result<(), String> {
    let (new, old) = (sibling(target, ".new"), sibling(target, ".old"));
    remove_any(&new);
    remove_any(&old);
    // Beside the target first, so the swap itself can't fail halfway across two disks.
    if fs::rename(staged, &new).is_err() {
        let copied = if staged.is_dir() {
            // ditto keeps a bundle's symlinks and signature intact.
            Command::new("ditto")
                .arg(staged)
                .arg(&new)
                .status()
                .map_err(|e| e.to_string())
                .and_then(|s| if s.success() { Ok(()) } else { Err(format!("ditto: {s}")) })
        } else {
            fs::copy(staged, &new).map(drop).map_err(|e| e.to_string())
        };
        copied.map_err(|e| format!("couldn't put the new version beside {}: {e}", target.display()))?;
    }
    if let Err(e) = fs::rename(target, &old) {
        remove_any(&new);
        return Err(format!("couldn't move {} aside: {e}", target.display()));
    }
    if let Err(e) = fs::rename(&new, target) {
        _ = fs::rename(&old, target);
        return Err(format!("couldn't move the new version into place: {e}"));
    }
    // A running executable can't be deleted on Windows; the new version removes it (see `finish`).
    if !cfg!(windows) {
        remove_any(&old);
    }
    Ok(())
}

fn start(target: &Path, args: &[&str]) -> Result<(), String> {
    let mut command = if cfg!(target_os = "macos") {
        // -n: a new instance, while this one is still quitting.
        let mut open = Command::new("open");
        open.arg("-n").arg(target).arg("--args");
        open
    } else {
        Command::new(target)
    };
    command.args(args);
    if let Some(dir) = target.parent() {
        command.current_dir(dir);
    }
    command.spawn().map(drop).map_err(|e| format!("couldn't start the new version: {e}"))
}

/// install.ps1's entry in Settings > Apps shows the version it installed; keeps it current.
#[cfg(windows)]
fn set_listed_version(target: &Path, version: &str) {
    use winreg::RegKey;
    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
    let Ok(key) =
        RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\CraftCraft", KEY_READ | KEY_WRITE)
    else {
        return;
    };
    let location: String = key.get_value("InstallLocation").unwrap_or_default();
    if target.parent().is_some_and(|d| d.to_string_lossy().eq_ignore_ascii_case(location.trim_end_matches('\\'))) {
        _ = key.set_value("DisplayVersion", &version);
    }
}

/// Runs the new .msi from a hidden PowerShell script, which waits for it, notes how it went for
/// [`finish`], and starts the launcher again (the new version, or the old one if it failed).
#[cfg(windows)]
fn run_msi(msi: &Path, paths: &Paths, for_everyone: bool, args: &[&str]) -> Result<(), String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    // The installer keeps the launcher where it is.
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let log = paths.logs().join(MSI_LOG);
    let result = update_dir(paths).join("result");
    let quote = |s: &str| s.replace('\'', "''");
    // `/passive` shows progress, and for everyone, Windows' request for approval.
    let msiexec = format!("/i \"{}\" /passive /norestart /l*v \"{}\"{}", msi.display(), log.display(), if for_everyone { " ALLUSERS=1" } else { "" });
    let restart = args.iter().map(|a| format!("'{a}'")).collect::<Vec<_>>().join(",");
    let script = format!(
        "$p = Start-Process msiexec.exe -ArgumentList '{}' -Wait -PassThru; Set-Content -LiteralPath '{}' -Value $p.ExitCode; \
         Start-Process -FilePath '{}' -ArgumentList {restart}",
        quote(&msiexec),
        quote(&result.to_string_lossy()),
        quote(&exe.to_string_lossy()),
    );
    debug::log(format_args!("running the update: {script}"));
    Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map(drop)
        .map_err(|e| format!("couldn't start the installer: {e}"))
}

/// At start-up: clears what an update left behind, and when this launcher was restarted by one,
/// says how it went: `Ok` with this version, or `Err` with why the installer failed.
pub fn finish(paths: &Paths) -> Option<Result<String, String>> {
    if let Ok(exe) = std::env::current_exe() {
        let target =
            if cfg!(target_os = "macos") { exe.ancestors().find(|p| p.extension().is_some_and(|e| e == "app")).map(Path::to_path_buf) } else { Some(exe) };
        if let Some(target) = target {
            remove_any(&sibling(&target, ".old"));
            remove_any(&sibling(&target, ".new"));
        }
    }
    let dir = update_dir(paths);
    let code = fs::read_to_string(dir.join("result")).ok().and_then(|c| c.trim().parse::<i32>().ok());
    _ = fs::remove_dir_all(&dir);
    if !std::env::args().any(|a| a == "--restarted") {
        return None;
    }
    let log = paths.logs().join(MSI_LOG);
    let outcome = match code {
        None | Some(0 | 1641 | 3010) => Ok(env!("CARGO_PKG_VERSION").to_string()),
        Some(1602) => Err("the update was cancelled".to_string()),
        Some(code) => {
            let reason = fs::read(&log).ok().and_then(|bytes| crate::install::msi_error(&crate::install::decode_log(&bytes)));
            Err(reason.map_or_else(|| format!("the installer stopped with code {code}"), |r| format!("{r} (code {code})")))
        }
    };
    if outcome.is_ok() && !debug::enabled() {
        _ = fs::remove_file(&log);
    }
    debug::log(format_args!("restarted after an update: {outcome:?}"));
    Some(outcome)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_this_computers_release_file() {
        let replace = How::Replace { target: PathBuf::from("x") };
        let name = asset_name(&replace).unwrap();
        assert!(name.starts_with("craftcraft-"));
        if cfg!(windows) {
            assert!(name.ends_with(".zip"));
            assert!(asset_name(&How::Msi { for_everyone: false }).unwrap().ends_with(".msi"));
        } else {
            assert!(name.ends_with(".tar.gz"));
        }
    }

    #[test]
    fn swaps_the_new_version_in() {
        let dir = std::env::temp_dir().join(format!("craftcraft-test-selfupdate-{}", std::process::id()));
        _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let (target, staged) = (dir.join("craftcraft"), dir.join("staged"));
        fs::write(&target, b"old").unwrap();
        fs::write(&staged, b"new").unwrap();
        replace(&staged, &target).unwrap();
        assert_eq!(fs::read(&target).unwrap(), b"new");
        assert!(!staged.exists() && !sibling(&target, ".new").exists());
        assert_eq!(sibling(&target, ".old").exists(), cfg!(windows));

        // Nothing staged: the old version stays.
        assert!(replace(&dir.join("missing"), &target).is_err());
        assert_eq!(fs::read(&target).unwrap(), b"new");
        _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn development_builds_are_updated_by_hand() {
        assert!(matches!(how(), How::Manual(_)) || !cfg!(debug_assertions));
    }
}
