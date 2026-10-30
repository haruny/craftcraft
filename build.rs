//! Sets `cfg(tray)` where there's a tray icon (Windows, macOS). On Windows, embeds the icon and
//! version into craftcraft.exe, so Explorer, the Start menu and Settings > Apps show them, and the
//! installer can read the version without running the app.

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // `tray`: the launcher has a tray (menu bar) icon and can keep running there.
    println!("cargo::rustc-check-cfg=cfg(tray)");
    if os == "windows" || os == "macos" {
        println!("cargo::rustc-cfg=tray");
    }
    if os == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico").set("ProductName", "CraftCraft").set("FileDescription", "CraftCraft");
        res.compile().expect("couldn't embed the Windows icon and version");
    }
}
