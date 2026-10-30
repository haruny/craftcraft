//! CraftCraft: installs, updates and opens well-known software rewritten in Rust.
//!
//! The list of apps isn't built in: it comes from `catalog/catalog.json`, published to GitHub Pages,
//! which the launcher checks at start-up and every few hours. See CONTRIBUTING.md for its format.

#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod catalog;
mod detect;
mod feed;
mod github;
mod http;
mod icons;
mod index;
mod install;
mod paths;
mod state;
mod system;
mod theme;
#[cfg(tray)]
mod tray;
mod ui;
mod version;
mod worker;

use eframe::egui;

/// Unix seconds.
pub fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version") => {
            println!("craftcraft {}", env!("CARGO_PKG_VERSION"));
            return std::process::ExitCode::SUCCESS;
        }
        Some("--check-catalog") => return check_catalog(args.get(1).map(String::as_str)),
        Some("--detect") => return detect(args.get(1).map(String::as_str)),
        // Started at logon: stay in the tray until asked for.
        Some("--background") | None => {}
        Some(other) => {
            eprintln!("unknown option {other}\nusage: craftcraft [--background | --version | --check-catalog <catalog.json> | --detect <catalog.json>]");
            return std::process::ExitCode::FAILURE;
        }
    }
    let background = args.first().is_some_and(|a| a == "--background");
    #[cfg(windows)]
    let Some(instance) = tray::Instance::claim(background) else {
        return std::process::ExitCode::SUCCESS;
    };

    let paths = match paths::Paths::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("CraftCraft")
            .with_app_id("io.craftcraft.CraftCraft")
            .with_inner_size([1080.0, 760.0])
            .with_min_inner_size([560.0, 420.0])
            .with_icon(icon())
            .with_active(!background),
        ..Default::default()
    };
    let launcher = move |cc: &eframe::CreationContext<'_>| {
        let launcher = ui::Launcher::new(cc, paths);
        #[cfg(tray)]
        let launcher = launcher.with_tray(
            &cc.egui_ctx,
            background,
            #[cfg(windows)]
            instance,
        );
        launcher
    };
    match eframe::run_native("CraftCraft", options, Box::new(|cc| Ok(Box::new(launcher(cc))))) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}

/// Parses a catalog the way the launcher will and reports what it would skip: run it on an edited
/// catalog.json before publishing.
fn check_catalog(path: Option<&str>) -> std::process::ExitCode {
    let Some(path) = path else {
        eprintln!("usage: craftcraft --check-catalog <catalog.json>");
        return std::process::ExitCode::FAILURE;
    };
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{path}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    match catalog::Catalog::parse(&text) {
        Ok(parsed) => {
            for p in &parsed.problems {
                println!("problem  {p}");
            }
            let platform = catalog::Platform::current();
            let here = platform.map_or(0, |p| parsed.catalog.apps.iter().filter(|a| a.asset_for(p).is_some()).count());
            println!(
                "{} apps, {} problems; {here} have a build for this computer ({})",
                parsed.catalog.apps.len(),
                parsed.problems.len(),
                platform.map_or_else(|| "unknown".into(), catalog::Platform::key)
            );
            if parsed.problems.is_empty() { std::process::ExitCode::SUCCESS } else { std::process::ExitCode::FAILURE }
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::FAILURE
        }
    }
}

/// Lists which of a catalog's apps are already on this computer, as the launcher sees them.
fn detect(path: Option<&str>) -> std::process::ExitCode {
    let (Some(path), Some(platform)) = (path, catalog::Platform::current()) else {
        eprintln!("usage: craftcraft --detect <catalog.json>");
        return std::process::ExitCode::FAILURE;
    };
    let parsed = match std::fs::read_to_string(path).map_err(|e| e.to_string()).and_then(|t| catalog::Catalog::parse(&t)) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{path}: {e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let paths = match paths::Paths::new() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{e}");
            return std::process::ExitCode::FAILURE;
        }
    };
    let found = detect::detect_all(&parsed.catalog.apps, platform, &paths);
    for app in &parsed.catalog.apps {
        if let Some(d) = found.get(&app.id) {
            let exe = d.exe.as_ref().map_or_else(|| "?".into(), |e| e.display().to_string());
            println!("{:<12} {:<10} {exe}", app.id, d.version.as_deref().unwrap_or("?"));
        }
    }
    println!("{} of {} apps found", found.len(), parsed.catalog.apps.len());
    std::process::ExitCode::SUCCESS
}

/// The window icon, drawn rather than shipped: an orange tile with two overlapping squares.
pub fn icon() -> egui::IconData {
    mark(64, true)
}

/// The two squares alone, black on clear, for the macOS menu bar to tint (18 points, at 2x).
#[cfg(target_os = "macos")]
pub fn menu_bar_icon() -> egui::IconData {
    mark(36, false)
}

/// The mark, `size` pixels square, on its tile or (zoomed in on the squares) without it.
fn mark(size: usize, tile: bool) -> egui::IconData {
    let mut rgba = vec![0u8; size * size * 4];
    // In the tile's 32 units, the squares cover 5.7 to 25.
    let (origin, span) = if tile { (0.0, 32.0) } else { (4.7, 21.3) };
    let unit = size as f32 / span;
    let inside = |x: f32, y: f32, x0: f32, y0: f32, size: f32, r: f32| {
        let (cx, cy) = (x.clamp(x0 + r, x0 + size - r), y.clamp(y0 + r, y0 + size - r));
        (x - cx).powi(2) + (y - cy).powi(2) <= r * r
    };
    for py in 0..size {
        for px in 0..size {
            let (x, y) = (origin + (px as f32 + 0.5) / unit, origin + (py as f32 + 0.5) / unit);
            let outer = inside(x, y, 5.7, 5.7, 13.6, 3.8);
            let inner = inside(x, y, 8.3, 8.3, 8.4, 1.2);
            let squares = (outer && !inner) || inside(x, y, 14.0, 14.0, 11.0, 2.5);
            let c = match (tile, squares) {
                (true, true) => [0xff, 0xff, 0xff, 0xff],
                (true, false) if inside(x, y, 0.0, 0.0, 32.0, 7.0) => [0xe8, 0x66, 0x2e, 0xff],
                (false, true) => [0, 0, 0, 0xff],
                _ => [0; 4],
            };
            rgba[(py * size + px) * 4..][..4].copy_from_slice(&c);
        }
    }
    egui::IconData { rgba, width: size as u32, height: size as u32 }
}
