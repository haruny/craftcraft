# CraftCraft

**Well-known software, rewritten in Rust: one launcher to install, update and open them.**

PhotoCraft instead of Photoshop, Zed instead of VS Code, BIMCraft instead of Revit, ripgrep instead
of grep. CraftCraft is a small native app (Rust and egui, no Electron) that lists the Rust rewrites
of the tools you already use, installs the right build for your computer, checks every download
against its published SHA-256, and tells you when there's an update or a new app in the catalog.

## The catalog drives everything

The list of apps isn't built into the launcher. It comes from
[`catalog/catalog.json`](catalog/catalog.json) in this repository, which is published at
`https://haruny.github.io/craftcraft/catalog.json` whenever it changes on `main`. Both the
launcher and the [website](https://haruny.github.io/craftcraft-site/) read it. The launcher fetches
it:

- at start-up, sending the last ETag so an unchanged catalog costs one tiny request;
- every `refresh_hours` (set in the catalog, 1–168) while it runs, retrying 15 minutes after a failure;
- when you press **Check for updates**.

It keeps the last good copy, so it opens instantly and works offline. When the catalog changes:

| Change in `catalog.json` | What the launcher shows |
|---|---|
| A new app in `apps` | A "new in the catalog" banner and a NEW badge, until the user clicks **Got it** |
| `launcher.version` newer than this build | An update banner with a download link |
| A new entry in `notices` | A banner until dismissed |
| A project publishes a release | **Update** on that app's card (each project's latest GitHub release is checked, also with ETags) |

**To add an app, send a pull request that adds it to `catalog/catalog.json`.**
[CONTRIBUTING.md](CONTRIBUTING.md) describes the format and how to check an entry.

The catalog address is fixed: release builds always read the published one, and there's no setting
to change it.

## Apps you already have

The launcher looks for every catalog app already on the computer, at start-up, after each catalog
check and after each install or uninstall. An app found this way shows as installed with its
version, gets **Open**, and gets **Update** when its project publishes a newer release.

| Where | How it's found | Version from |
|---|---|---|
| Windows desktop apps | The list of installed programs (the registry's Uninstall entries), matched strictly by name: `PhotoCraft`, `PhotoCraft 0.16.0` and `DesignCraft (Craft Apps)` match, `ArtCraft Launcher` doesn't match ArtCraft | `DisplayVersion` |
| macOS desktop apps | `/Applications/<Name>.app`, `~/Applications/<Name>.app` | The bundle's `Info.plist` |
| Linux desktop apps | The catalog's `launch` path, the command on PATH, or an AppImage in `~/Applications` | dpkg or rpm, or the AppImage's file name |
| Command-line and terminal apps (everywhere) | The command on PATH | Running `<command> --version` (hidden, at most 3 seconds) |

Updating an app installed by its own installer runs the new release's installer, which upgrades it
in place (MSI packages run with `/passive`). A command-line app installed some other way (winget,
cargo, a script) can't be replaced, so **Install latest** puts an up-to-date copy in the launcher's
folder instead. **Uninstall…** runs the app's own uninstaller.

`craftcraft --detect <catalog.json>` prints what the launcher finds.

## How apps are installed

Files come only from each app's own GitHub release (`https://github.com/<repo>/releases/download/…`)
and are verified against the SHA-256 GitHub publishes for every release file. A file without one is
installed but marked **unverified**.

| Release file | What happens |
|---|---|
| `.zip`, `.tar.gz`, `.tar.xz` | Unpacked into the launcher's `apps/<id>` folder. Updates unpack beside it and swap in only when complete, so a failed update leaves the old version working |
| `.AppImage`, single `.exe` | Kept as one executable in `apps/<id>` |
| `.msi`, `.exe` installer, `.dmg`, `.pkg`, `.deb`, `.rpm` | Handed to the system installer. The catalog's `launch` path is used to find and open it |

Terminal and command-line apps also get a launcher in the `bin` folder (a `.cmd` file on Windows, a
symlink elsewhere). **Settings → Add it to my PATH** puts that folder on your PATH on Windows; on
macOS and Linux, Settings shows the line to add to your shell profile. **Open in terminal** opens a
terminal with the folder already on its PATH.

Everything lives in one data folder: `%LOCALAPPDATA%\CraftCraft\CraftCraft\data` on Windows,
`~/Library/Application Support/io.CraftCraft.CraftCraft` on macOS, `~/.local/share/craftcraft` on
Linux, or `$CRAFTCRAFT_HOME` when set.

## Build and run

You need Rust 1.90 or newer. On Linux, install the windowing headers first (Debian/Ubuntu:
`sudo apt install libxkbcommon-dev libwayland-dev libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev`).

```sh
cargo run --release
cargo test                   # unit tests
cargo test -- --ignored      # also installs the real ripgrep from GitHub into a temp folder
```

| Environment variable | Use |
|---|---|
| `CRAFTCRAFT_FEED` | **Debug builds only:** read the catalog from another address or a local file, e.g. `catalog/catalog.json` while editing it. Release builds ignore it |
| `CRAFTCRAFT_HOME` | Use another data folder (handy for testing with a clean slate) |
| `GITHUB_TOKEN` | Raises GitHub's limit from 60 to 5,000 requests an hour |

`craftcraft --check-catalog <catalog.json>` parses a catalog exactly as the launcher will and lists
anything it would skip.

## Layout

```
catalog/         catalog.json, its JSON Schema, and check.mjs, which CI runs on every change
src/catalog.rs   the catalog format: parsing, validation, platforms, picking a release file
src/feed.rs      fetching and caching the catalog
src/github.rs    each app's latest release, ETag-cached, rate-limit aware
src/detect.rs    finding apps already installed (registry, app bundles, packages, PATH)
src/install.rs   download, verify, unpack, swap in, shims, uninstall
src/system.rs    opening apps, terminals, folders and links; PATH
src/icons.rs     app icons, cached and decoded off the UI thread
src/worker.rs    background jobs and the events they send the UI
src/ui.rs        the window
src/theme.rs     colours, fonts and widget styling
```

## License

MIT OR Apache-2.0. The bundled Geist fonts (`assets/fonts`) are under the SIL Open Font License
(`assets/fonts/OFL.txt`). Each app the launcher installs is under its own license.
