//! Downloading, verifying, installing and removing apps.
//!
//! Every download is checked against the SHA-256 GitHub publishes for it. Archives are unpacked
//! into a staging folder and swapped in only once complete, so a failed update leaves the old
//! version working. Installers (.msi, .exe, .dmg, .pkg, .deb, .rpm) are handed to the system.

use std::fmt;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::catalog::{App, Kind, Method, Os, Platform};
use crate::github::Release;
use crate::paths::Paths;
use crate::state::Installed;
use crate::{debug, now};

#[derive(Clone, Copy, Debug)]
pub enum Stage {
    Downloading {
        done: u64,
        total: Option<u64>,
    },
    Installing,
    /// Waiting for the system installer the user is clicking through.
    Installer,
}

/// The step an install (or opening or removing an app) failed at, which the card names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Finding this computer's file in the release.
    Find,
    Download,
    Checksum,
    Unpack,
    /// Putting the app in its folder, and its commands in `bin`.
    Setup,
    /// The system installer.
    Installer,
    Open,
    Uninstall,
}

impl Step {
    pub fn failed(self) -> &'static str {
        match self {
            Step::Find => "No download",
            Step::Download => "Download failed",
            Step::Checksum => "Checksum failed",
            Step::Unpack => "Unpacking failed",
            Step::Setup => "Install failed",
            Step::Installer => "Installer failed",
            Step::Open => "Couldn't open",
            Step::Uninstall => "Uninstall failed",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Failure {
    pub step: Step,
    pub message: String,
    /// The system installer's log, when it kept one.
    pub log: Option<PathBuf>,
}

impl Failure {
    pub fn new(step: Step, message: impl Into<String>) -> Self {
        Self { step, message: message.into(), log: None }
    }
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.step.failed(), self.message)
    }
}

/// Turns an error from `step` into a [`Failure`], for `map_err`.
fn at(step: Step) -> impl Fn(String) -> Failure {
    move |message| Failure::new(step, message)
}

pub fn install(
    paths: &Paths,
    http: &crate::http::Http,
    app: &App,
    release: &Release,
    platform: Platform,
    for_everyone: bool,
    mut stage: impl FnMut(Stage),
) -> Result<Installed, Failure> {
    let (spec, method) = app.asset_for(platform).ok_or_else(|| format!("{} has no build for {}", app.name, platform.key())).map_err(at(Step::Find))?;
    let asset = release.pick(spec.pattern()).map_err(at(Step::Find))?;
    if asset.name.is_empty() || asset.name.contains(['/', '\\']) || asset.name.starts_with('.') {
        return Err(Failure::new(Step::Find, format!("refusing a download named {:?}", asset.name)));
    }

    let part = paths.downloads().join(format!("{}.part", asset.name));
    let file = paths.downloads().join(&asset.name);
    debug::log(format_args!("{} {}: downloading {}", app.id, release.tag, asset.url));
    let sha = http
        .download(&asset.url, &part, |done, total| stage(Stage::Downloading { done, total: total.or((asset.size > 0).then_some(asset.size)) }))
        .inspect_err(|_| _ = fs::remove_file(&part))
        .map_err(at(Step::Download))?;
    let verified = match &asset.sha256 {
        Some(expected) if *expected == sha => true,
        Some(expected) => {
            _ = fs::remove_file(&part);
            return Err(Failure::new(Step::Checksum, format!("{} failed its checksum (expected {expected}, got {sha}); it was deleted", asset.name)));
        }
        None => false,
    };
    debug::log(format_args!("{}: SHA-256 {sha}, {}", asset.name, if verified { "as published" } else { "none published" }));
    fs::rename(&part, &file).map_err(|e| format!("{}: {e}", file.display())).map_err(at(Step::Download))?;

    stage(Stage::Installing);
    let (path, bins) = match method {
        Method::Archive => {
            let dir = swap_in(paths, &app.id, Step::Unpack, |staging| extract(&file, &asset.name, staging))?;
            _ = fs::remove_file(&file);
            let bins = find_bins(&dir, &app.bin, platform.os);
            if bins.is_empty() {
                return Err(Failure::new(Step::Unpack, format!("unpacked {}, but couldn't find {} in it", asset.name, app.bin.join(", "))));
            }
            (dir, bins)
        }
        Method::Portable => {
            let name = asset.name.clone();
            let dir = swap_in(paths, &app.id, Step::Setup, |staging| {
                fs::create_dir_all(staging).map_err(|e| e.to_string())?;
                fs::rename(&file, staging.join(&name)).or_else(|_| fs::copy(&file, staging.join(&name)).map(drop)).map_err(|e| e.to_string())?;
                make_executable(&staging.join(&name))
            })?;
            _ = fs::remove_file(&file);
            let bin = dir.join(&name);
            (bin.clone(), vec![bin])
        }
        Method::Installer => {
            stage(Stage::Installer);
            let log = paths.logs().join(Path::new(&asset.name).with_extension("log"));
            let result = run_installer(&file, platform.os, for_everyone, &log);
            // Windows' installers have finished when they return (and Windows Installer keeps its own
            // copy). Elsewhere the user is still in the window that opened; tidy() clears it later.
            if platform.os == Os::Windows && !debug::enabled() {
                _ = fs::remove_file(&file);
            }
            result?;
            (file, Vec::new())
        }
        Method::Unknown => return Err(Failure::new(Step::Find, "this launcher doesn't know how to install that file; update CraftCraft")),
    };
    if app.kind != Kind::Gui {
        write_shims(paths, &app.id, &bins).map_err(at(Step::Setup))?;
    }
    Ok(Installed { tag: release.tag.clone(), method, path, bins, verified, installed_at: now() })
}

/// Removes an app the launcher unpacked. System-installed apps are removed through the system.
pub fn uninstall(paths: &Paths, id: &str, installed: &Installed) -> Result<(), String> {
    if installed.method == Method::Installer {
        return Err("installed with the system installer; remove it from your system's app settings".into());
    }
    remove_shims(paths, id);
    let dir = paths.app_dir(id);
    if dir.exists() {
        fs::remove_dir_all(&dir).map_err(|e| format!("couldn't remove {}: {e}. Is it still running?", dir.display()))?;
    }
    Ok(())
}

/// Builds the new version in a staging folder with `fill` (whose errors are `step`'s), then swaps it
/// in for the old one.
fn swap_in(paths: &Paths, id: &str, step: Step, fill: impl FnOnce(&Path) -> Result<(), String>) -> Result<PathBuf, Failure> {
    let target = paths.app_dir(id);
    let staging = paths.apps().join(format!(".{id}.new"));
    let old = paths.apps().join(format!(".{id}.old"));
    for leftover in [&staging, &old] {
        if leftover.exists() {
            fs::remove_dir_all(leftover).map_err(|e| Failure::new(Step::Setup, format!("couldn't clear {}: {e}", leftover.display())))?;
        }
    }
    if let Err(e) = fill(&staging) {
        _ = fs::remove_dir_all(&staging);
        return Err(Failure::new(step, e));
    }
    let root = single_folder(&staging).unwrap_or_else(|| staging.clone());

    if target.exists() {
        fs::rename(&target, &old).map_err(|_| Failure::new(Step::Setup, "couldn't replace the installed version. Close the app and try again."))?;
    }
    if let Err(e) = fs::rename(&root, &target) {
        if old.exists() {
            _ = fs::rename(&old, &target);
        }
        _ = fs::remove_dir_all(&staging);
        return Err(Failure::new(Step::Setup, format!("couldn't move the new version into place: {e}")));
    }
    _ = fs::remove_dir_all(&staging);
    _ = fs::remove_dir_all(&old);
    Ok(target)
}

/// Archives usually hold one top-level folder (`ripgrep-15.2.0-x86_64-pc-windows-msvc/`); its
/// contents become the app folder. A macOS bundle (`Ruffle.app`) is kept whole.
fn single_folder(dir: &Path) -> Option<PathBuf> {
    let mut entries = fs::read_dir(dir).ok()?.filter_map(Result::ok);
    let only = entries.next()?;
    if entries.next().is_some() || !only.file_type().ok()?.is_dir() || only.file_name().to_string_lossy().ends_with(".app") {
        return None;
    }
    Some(only.path())
}

pub(crate) fn extract(archive: &Path, name: &str, dest: &Path) -> Result<(), String> {
    fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    let lower = name.to_ascii_lowercase();
    let open = || File::open(archive).map(BufReader::new).map_err(|e| e.to_string());
    if lower.ends_with(".zip") {
        extract_zip(open()?, dest)
    } else if lower.ends_with(".tar.gz") || lower.ends_with(".tgz") {
        untar(tar::Archive::new(flate2::read::GzDecoder::new(open()?)), dest)
    } else if lower.ends_with(".tar.xz") || lower.ends_with(".txz") {
        let tar_path = archive.with_extension("tar.tmp");
        {
            let mut out = BufWriter::new(File::create(&tar_path).map_err(|e| e.to_string())?);
            lzma_rs::xz_decompress(&mut open()?, &mut out).map_err(|e| format!("couldn't decompress {name}: {e}"))?;
        }
        let result = untar(tar::Archive::new(open_file(&tar_path)?), dest);
        _ = fs::remove_file(&tar_path);
        result
    } else {
        Err(format!("don't know how to unpack {name}"))
    }
}

fn open_file(path: &Path) -> Result<BufReader<File>, String> {
    File::open(path).map(BufReader::new).map_err(|e| e.to_string())
}

fn untar<R: std::io::Read>(mut archive: tar::Archive<R>, dest: &Path) -> Result<(), String> {
    archive.set_preserve_permissions(true);
    // `unpack` refuses entries that would land outside `dest`.
    archive.unpack(dest).map_err(|e| format!("couldn't unpack: {e}"))
}

fn extract_zip(reader: BufReader<File>, dest: &Path) -> Result<(), String> {
    let mut zip = zip::ZipArchive::new(reader).map_err(|e| format!("not a valid zip: {e}"))?;
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| e.to_string())?;
        // `enclosed_name` is `None` for paths that would escape `dest` (`../x`, `/etc/x`).
        let Some(rel) = entry.enclosed_name() else {
            return Err(format!("the zip has an unsafe path: {}", entry.name().map(|n| n.into_owned()).unwrap_or_default()));
        };
        let out = dest.join(rel);
        if entry.is_dir() {
            fs::create_dir_all(&out).map_err(|e| e.to_string())?;
            continue;
        }
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut file = File::create(&out).map_err(|e| format!("{}: {e}", out.display()))?;
        std::io::copy(&mut entry, &mut file).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        if let Some(mode) = entry.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            _ = fs::set_permissions(&out, fs::Permissions::from_mode(mode & 0o777));
        }
    }
    Ok(())
}

/// The executables named in the catalog, shallowest match first.
fn find_bins(dir: &Path, names: &[String], os: Os) -> Vec<PathBuf> {
    names
        .iter()
        .filter_map(|name| {
            let file = if os == Os::Windows && !name.to_ascii_lowercase().ends_with(".exe") { format!("{name}.exe") } else { name.clone() };
            find_file(dir, &file)
        })
        .collect()
}

fn find_file(dir: &Path, name: &str) -> Option<PathBuf> {
    let mut level = vec![dir.to_path_buf()];
    for _ in 0..6 {
        let mut next = Vec::new();
        for d in &level {
            let Ok(entries) = fs::read_dir(d) else { continue };
            for e in entries.filter_map(Result::ok) {
                let path = e.path();
                let Ok(ft) = e.file_type() else { continue };
                if ft.is_file() && e.file_name().to_string_lossy().eq_ignore_ascii_case(name) {
                    return Some(path);
                }
                if ft.is_dir() {
                    next.push(path);
                }
            }
        }
        next.sort();
        level = next;
    }
    None
}

#[allow(clippy::unnecessary_wraps)]
fn make_executable(path: &Path) -> Result<(), String> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).map_err(|e| e.to_string())?;
    }
    let _ = path;
    Ok(())
}

/// Puts a launcher for each command into the shared `bin` folder: a `.cmd` file on Windows (it
/// works from cmd and PowerShell, and keeps the app beside its own files), a symlink elsewhere.
fn write_shims(paths: &Paths, id: &str, bins: &[PathBuf]) -> Result<(), String> {
    remove_shims(paths, id);
    for bin in bins {
        let Some(stem) = bin.file_stem().map(|s| s.to_string_lossy().into_owned()) else { continue };
        if cfg!(windows) {
            let shim = paths.bin().join(format!("{stem}.cmd"));
            let body = format!("@echo off\r\nrem craftcraft:{id}\r\n\"{}\" %*\r\n", bin.display());
            fs::write(&shim, body).map_err(|e| format!("{}: {e}", shim.display()))?;
        } else {
            #[cfg(unix)]
            {
                let shim = paths.bin().join(&stem);
                _ = fs::remove_file(&shim);
                std::os::unix::fs::symlink(bin, &shim).map_err(|e| format!("{}: {e}", shim.display()))?;
            }
        }
    }
    Ok(())
}

/// Removes the shims that point into this app's folder (and no one else's).
fn remove_shims(paths: &Paths, id: &str) {
    let app_dir = paths.app_dir(id);
    let Ok(entries) = fs::read_dir(paths.bin()) else { return };
    for e in entries.filter_map(Result::ok) {
        let path = e.path();
        let ours = if cfg!(windows) {
            fs::read_to_string(&path).is_ok_and(|s| s.contains(&format!("rem craftcraft:{id}\r\n")))
        } else {
            fs::read_link(&path).is_ok_and(|target| target.starts_with(&app_dir))
        };
        if ours {
            _ = fs::remove_file(&path);
        }
    }
}

/// Runs `file` and waits for it. An .msi logs to `log`, which is where the reason for a failure is
/// found; the log is kept only then (or in debug mode).
fn run_installer(file: &Path, os: Os, for_everyone: bool, log: &Path) -> Result<(), Failure> {
    let msi = os == Os::Windows && file.to_string_lossy().to_ascii_lowercase().ends_with(".msi");
    let mut command = match os {
        // `/passive` shows progress without wizard pages; an installed older version is upgraded in place.
        // An installer that can go either way installs for the current user unless told otherwise, and
        // won't upgrade a copy installed for everyone from there.
        Os::Windows if msi => {
            let mut msiexec = Command::new("msiexec");
            msiexec.arg("/i").arg(file).arg("/passive").arg("/l*v").arg(log);
            if for_everyone {
                msiexec.arg("ALLUSERS=1");
            }
            msiexec
        }
        Os::Windows => Command::new(file),
        // These open the disk image, the macOS Installer, or the Linux software centre, and
        // return straight away; the user finishes the install there.
        Os::Macos => Command::new("open"),
        Os::Linux => Command::new("xdg-open"),
    };
    if !matches!(os, Os::Windows) {
        command.arg(file);
    }
    debug::log(format_args!("running {command:?}"));
    let status = command.status().map_err(|e| Failure::new(Step::Installer, format!("couldn't start the installer: {e}")))?;
    debug::log(format_args!("the installer exited: {status}"));
    let code = status.code();
    // 1641 and 3010: installed, and a restart is needed (or under way).
    if matches!(code, Some(0 | 1641 | 3010)) {
        if msi && !debug::enabled() {
            _ = fs::remove_file(log);
        }
        return Ok(());
    }
    let reason = if msi { fs::read(log).ok().and_then(|bytes| msi_error(&decode_log(&bytes))) } else { None };
    let message = match (code, reason) {
        (Some(1602), _) => "the installation was cancelled".to_string(),
        (Some(code), Some(reason)) => format!("{reason} (code {code})"),
        (Some(code), None) => exit_meaning(code).map_or_else(|| format!("the installer stopped with code {code}"), |m| format!("{m} (code {code})")),
        (None, _) => "the installer was stopped".to_string(),
    };
    Err(Failure { step: Step::Installer, message, log: Some(log.to_path_buf()).filter(|l| msi && l.is_file()) })
}

/// Why Windows Installer failed, from its log: the reason it gives on a `Product: <name> -- <reason>`
/// line (which also goes to the event log) ahead of the "Installation failed." that follows, or
/// failing that, an `Error 1925. …` line.
pub(crate) fn msi_error(log: &str) -> Option<String> {
    const OUTCOMES: [&str; 4] = ["Installation failed.", "Installation operation failed.", "Removal failed.", "Configuration failed."];
    let product = log
        .lines()
        .filter_map(|line| line.split_once("Product: ")?.1.split_once(" -- ").map(|(_, reason)| reason.trim()))
        .find(|reason| !reason.is_empty() && !reason.ends_with("completed successfully.") && !OUTCOMES.contains(reason));
    let error = || log.lines().map(str::trim).find(|line| line.strip_prefix("Error ").is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_digit())));
    let reason = product.or_else(error)?;
    Some(if reason.chars().count() > 400 { format!("{}…", reason.chars().take(400).collect::<String>()) } else { reason.to_string() })
}

/// Windows Installer writes its logs in UTF-16.
pub(crate) fn decode_log(bytes: &[u8]) -> String {
    match bytes.strip_prefix(&[0xff, 0xfe]) {
        Some(utf16) => String::from_utf16_lossy(&utf16.as_chunks::<2>().0.iter().map(|&pair| u16::from_le_bytes(pair)).collect::<Vec<_>>()),
        None => String::from_utf8_lossy(bytes).into_owned(),
    }
}

/// What Windows Installer's exit codes mean, for when its log doesn't say.
fn exit_meaning(code: i32) -> Option<&'static str> {
    Some(match code {
        1603 => "the installer ran into an error",
        1618 => "another installation is under way; try again once it's done",
        1619 | 1620 => "Windows couldn't open the installer",
        1625 => "Windows didn't allow this installation; it may need an administrator's approval",
        1633 => "this installer isn't for this computer's processor",
        1638 => "another version of this app is installed; uninstall it first",
        _ => return None,
    })
}

/// Clears out interrupted work, installers left behind, and installers' logs older than a week.
pub fn tidy(paths: &Paths) {
    if let Ok(entries) = fs::read_dir(paths.apps()) {
        for e in entries.filter_map(Result::ok) {
            let name = e.file_name().to_string_lossy().into_owned();
            if name.starts_with('.') && (name.ends_with(".new") || name.ends_with(".old")) {
                _ = fs::remove_dir_all(e.path());
            }
        }
    }
    let week = std::time::Duration::from_secs(7 * 24 * 3600);
    let older = |e: &fs::DirEntry| e.metadata().and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok()).is_some_and(|age| age > week);
    // Only one launcher runs on Windows, and its installers have finished when they return, so
    // anything there now is left over. Elsewhere a disk image may still be open.
    let leftovers = cfg!(windows) && !debug::enabled();
    if let Ok(entries) = fs::read_dir(paths.downloads()) {
        for e in entries.filter_map(Result::ok) {
            let partial = e.file_name().to_string_lossy().ends_with(".part");
            if leftovers || partial || older(&e) {
                _ = fs::remove_file(e.path());
            }
        }
    }
    // The launcher's own log starts over by itself.
    if let Ok(entries) = fs::read_dir(paths.logs()) {
        for e in entries.filter_map(Result::ok) {
            if !e.file_name().to_string_lossy().starts_with("craftcraft.") && older(&e) {
                _ = fs::remove_file(e.path());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp_paths(name: &str) -> Paths {
        let root = std::env::temp_dir().join(format!("craftcraft-test-{name}-{}", std::process::id()));
        _ = fs::remove_dir_all(&root);
        let paths = Paths { root };
        for d in [paths.apps(), paths.bin(), paths.downloads(), paths.cache(), paths.logs()] {
            fs::create_dir_all(d).unwrap();
        }
        paths
    }

    fn make_zip(path: &Path, files: &[(&str, &[u8])]) {
        let mut zip = zip::ZipWriter::new(File::create(path).unwrap());
        for (name, body) in files {
            zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            zip.write_all(body).unwrap();
        }
        zip.finish().unwrap();
    }

    #[test]
    fn unpacks_flattens_finds_bins_and_swaps() {
        let paths = temp_paths("swap");
        let zip = paths.downloads().join("rg.zip");
        make_zip(&zip, &[("ripgrep-1/rg.exe", b"one"), ("ripgrep-1/doc/README", b"r")]);
        let dir = swap_in(&paths, "ripgrep", Step::Unpack, |s| extract(&zip, "rg.zip", s)).unwrap();
        assert_eq!(dir, paths.app_dir("ripgrep"));
        assert_eq!(find_bins(&dir, &["rg".into()], Os::Windows), [dir.join("rg.exe")]);
        assert_eq!(find_bins(&dir, &["rg".into()], Os::Linux), Vec::<PathBuf>::new());

        // An update replaces the folder; a failed one leaves it alone.
        make_zip(&zip, &[("ripgrep-2/rg.exe", b"two")]);
        swap_in(&paths, "ripgrep", Step::Unpack, |s| extract(&zip, "rg.zip", s)).unwrap();
        assert_eq!(fs::read(dir.join("rg.exe")).unwrap(), b"two");
        assert!(!dir.join("doc").exists());
        assert_eq!(swap_in(&paths, "ripgrep", Step::Unpack, |_| Err("boom".into())).unwrap_err().step, Step::Unpack);
        assert_eq!(fs::read(dir.join("rg.exe")).unwrap(), b"two");
        _ = fs::remove_dir_all(&paths.root);
    }

    #[test]
    fn refuses_zip_slip() {
        let paths = temp_paths("slip");
        let zip = paths.downloads().join("evil.zip");
        make_zip(&zip, &[("../../evil.txt", b"x")]);
        assert!(extract(&zip, "evil.zip", &paths.apps().join("x")).is_err());
        assert!(!paths.root.join("evil.txt").exists());
        _ = fs::remove_dir_all(&paths.root);
    }

    #[test]
    fn keeps_app_bundles_whole() {
        let paths = temp_paths("bundle");
        let zip = paths.downloads().join("r.zip");
        make_zip(&zip, &[("Ruffle.app/Contents/MacOS/ruffle", b"x")]);
        let dir = swap_in(&paths, "ruffle", Step::Unpack, |s| extract(&zip, "r.zip", s)).unwrap();
        assert!(dir.join("Ruffle.app/Contents/MacOS/ruffle").exists());
        _ = fs::remove_dir_all(&paths.root);
    }

    /// The whole path against the real GitHub: `cargo test -- --ignored`.
    #[test]
    #[ignore = "downloads ripgrep from GitHub"]
    fn installs_ripgrep_from_github() {
        let paths = temp_paths("live");
        let text = r#"{"schema":1,"name":"t","launcher":{"version":"0.1.0","repo":"o/l"},"apps":[{
            "id":"ripgrep","name":"ripgrep","replaces":["grep"],"category":"cli","kind":"cli","tagline":"t",
            "repo":"BurntSushi/ripgrep","bin":["rg"],"assets":{
              "windows-x86_64":"ripgrep-*-x86_64-pc-windows-msvc.zip","windows-aarch64":"ripgrep-*-aarch64-pc-windows-msvc.zip",
              "macos-x86_64":"ripgrep-*-x86_64-apple-darwin.tar.gz","macos-aarch64":"ripgrep-*-aarch64-apple-darwin.tar.gz",
              "linux-x86_64":"ripgrep-*-x86_64-unknown-linux-musl.tar.gz","linux-aarch64":"ripgrep-*-aarch64-unknown-linux-musl.tar.gz"}}]}"#;
        let catalog = crate::catalog::Catalog::parse(text).unwrap().catalog;
        let app = catalog.app("ripgrep").unwrap();
        let http = crate::http::Http::new();
        let releases = crate::github::Releases::new(http.clone(), paths.cache().join("releases.json"));
        let crate::github::Lookup::Found(release) = releases.latest(&app.repo) else { panic!("no ripgrep release") };

        let mut stages = 0;
        let installed = install(&paths, &http, app, &release, Platform::current().unwrap(), false, |_| stages += 1).unwrap();
        assert!(stages > 1);
        assert!(installed.verified, "GitHub publishes ripgrep's SHA-256");
        assert_eq!(installed.tag, release.tag);
        let out = Command::new(&installed.bins[0]).arg("--version").output().unwrap();
        assert!(String::from_utf8_lossy(&out.stdout).starts_with("ripgrep "));
        assert_eq!(fs::read_dir(paths.bin()).unwrap().count(), 1, "one shim for rg");
        assert_eq!(fs::read_dir(paths.downloads()).unwrap().count(), 0, "the download is cleaned up");

        uninstall(&paths, "ripgrep", &installed).unwrap();
        assert!(!paths.app_dir("ripgrep").exists());
        assert_eq!(fs::read_dir(paths.bin()).unwrap().count(), 0);
        _ = fs::remove_dir_all(&paths.root);
    }

    #[test]
    fn shims_belong_to_their_app() {
        let paths = temp_paths("shims");
        let bin = paths.app_dir("uv").join(if cfg!(windows) { "uv.exe" } else { "uv" });
        fs::create_dir_all(bin.parent().unwrap()).unwrap();
        fs::write(&bin, b"x").unwrap();
        write_shims(&paths, "uv", std::slice::from_ref(&bin)).unwrap();
        assert_eq!(fs::read_dir(paths.bin()).unwrap().count(), 1);
        remove_shims(&paths, "ripgrep");
        assert_eq!(fs::read_dir(paths.bin()).unwrap().count(), 1);
        remove_shims(&paths, "uv");
        assert_eq!(fs::read_dir(paths.bin()).unwrap().count(), 0);
        _ = fs::remove_dir_all(&paths.root);
    }

    /// As in Windows Installer's log of DeckCraft refusing to update a copy installed for everyone.
    const REFUSED: &str = "=== Verbose logging started: 10/30/2026  21:13:48  Build type: SHIP UNICODE 5.00.10011.00 ===\r\n\
        ******* Product: C:\\Users\\me\\AppData\\Local\\CraftCraft\\CraftCraft\\data\\downloads\\deckcraft-0.12.0-windows-x64.msi\r\n\
        Action start 21:13:49: LaunchConditions.\r\n\
        MSI (s) (F4:E8) [21:13:49:112]: Product: DeckCraft -- This app is already installed for everyone. Choose Everyone to update it.\r\n\
        This app is already installed for everyone. Choose Everyone to update it.\r\n\
        Action ended 21:13:49: LaunchConditions. Return value 3.\r\n\
        MSI (s) (F4:E8) [21:13:49:121]: Product: DeckCraft -- Installation failed.\r\n";

    #[test]
    fn reads_why_windows_installer_failed() {
        let utf16: Vec<u8> = [0xff, 0xfe].into_iter().chain(REFUSED.encode_utf16().flat_map(u16::to_le_bytes)).collect();
        let log = decode_log(&utf16);
        assert_eq!(log, REFUSED);
        assert_eq!(msi_error(&log).as_deref(), Some("This app is already installed for everyone. Choose Everyone to update it."));

        let privileges = "MSI (s) (10:20) [09:00:00:000]: Product: Old -- Removal completed successfully.\r\n\
            Error 1925. You do not have sufficient privileges to complete this installation for all users of the machine.\r\n\
            MSI (s) (10:20) [09:00:01:000]: Product: X -- Installation operation failed.\r\n";
        assert_eq!(
            msi_error(privileges).as_deref(),
            Some("Error 1925. You do not have sufficient privileges to complete this installation for all users of the machine.")
        );
        assert_eq!(msi_error("MSI (s) (10:20) [09:00:01:000]: Product: X -- Installation failed.\r\n"), None);
    }

    #[test]
    fn steps_name_the_failure() {
        let f = Failure::new(Step::Installer, "the installation was cancelled");
        assert_eq!(f.to_string(), "Installer failed: the installation was cancelled");
    }
}
