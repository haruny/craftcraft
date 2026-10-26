//! On Windows, embeds the icon and version into craftcraft.exe, so Explorer, the Start menu and
//! Settings > Apps show them, and the installer can read the version without running the app.

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico").set("ProductName", "CraftCraft").set("FileDescription", "CraftCraft");
        res.compile().expect("couldn't embed the Windows icon and version");
    }
}
