# CraftCraft

**English** · [Français](README.fr.md) · [Español](README.es.md) · [中文](README.zh-CN.md) · [हिन्दी](README.hi.md) · [العربية](README.ar.md) · [বাংলা](README.bn.md) · [Português](README.pt.md)

[![Downloads](https://img.shields.io/github/downloads/haruny/craftcraft/total?color=e8662e)](https://github.com/haruny/craftcraft/releases)
[![Latest release](https://img.shields.io/github/v/release/haruny/craftcraft?color=e8662e)](https://github.com/haruny/craftcraft/releases/latest)

**Well-known software, rewritten in Rust: one launcher to install, update and open them.**

PhotoCraft instead of Photoshop, GridCraft instead of Excel, BIMCraft instead of Revit, RustDesk
instead of TeamViewer. CraftCraft is a small native app (Rust and egui, no Electron) that lists
open-source Rust replacements for the paid software you already use, installs the right build for your computer, checks every download
against its published SHA-256, and tells you when there's an update or a new app in the catalog.

## Install

```powershell
# Windows
irm https://haruny.github.io/craftcraft/install.ps1 | iex
```
```sh
# macOS and Linux
curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
```

The script downloads the latest release for your system from
[Releases](https://github.com/haruny/craftcraft/releases), checks its SHA-256, installs it for
your user only (no admin rights), and opens it. Run it again to update.

| | Installed to | Also |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Programs\CraftCraft` | Start menu shortcut; listed in Settings → Apps, which uninstalls it; a tray icon while it runs |
| macOS | `~/Applications/CraftCraft.app` | One build for Apple silicon and Intel |
| Linux | `~/.local/bin/craftcraft` | Applications menu entry and icon |

Or download an installer from [Releases](https://github.com/haruny/craftcraft/releases/latest):

- **Windows**: `craftcraft-windows-x86_64.msi` (or `-aarch64.msi` for Arm). It asks who CraftCraft
  is for: **Only for me** installs it the same way as the script (no admin rights), **Everyone**
  installs it in `%ProgramFiles%\CraftCraft` and every user's Start menu, after Windows asks for
  an administrator's approval. Then it offers to open CraftCraft. Run a newer one to update; it
  has to make the same choice as the copy it replaces (uninstall that copy to switch). Without its
  window (`msiexec /i … /qn`) it installs for the current user, or for everyone with `ALLUSERS=1`.
  For a copy that's just for you, the script and the installer can take each other's place:
  whichever ran last owns the install, and Settings → Apps lists it once.
- **macOS**: `craftcraft-macos-universal.dmg`. Open it and drag CraftCraft to Applications.

On Windows and macOS, **Settings** also has:

- **Start on logon**: CraftCraft starts in the tray (the menu bar on macOS) when you sign in, as
  `craftcraft --background`. Windows runs it from the `CraftCraft` value under
  `HKCU\Software\Microsoft\Windows\CurrentVersion\Run`; macOS from
  `~/Library/LaunchAgents/io.craftcraft.CraftCraft.plist`.
- **Minimize to tray on close** (on by default): closing the window leaves CraftCraft in the tray,
  still checking for updates; the icon's tooltip counts them. On Windows, click the icon to open
  the window, or right-click it to check now or quit. On macOS, click it for the same menu; while
  the window is closed there's no Dock icon, and opening CraftCraft from Finder brings the window
  back. ⌘Q still quits.

Only one copy runs: opening CraftCraft again brings back the window of the one in the tray. On
Windows, a copy that wasn't installed by the script or the installer (unpacked from the zip, say)
adds itself to the Start menu.

**Settings → Debug mode** (everywhere) logs what CraftCraft does to `logs/craftcraft.log` in the data
folder, and keeps downloaded installers and their logs instead of deleting them.

## Languages

CraftCraft is in English unless you pick another language in **Settings → Language**: Français,
Español, 中文（简体）, हिन्दी, العربية, বাংলা or Português. The change applies straight away, tray
menu included. Text that comes from the catalog (app taglines and descriptions, categories,
notices) stays in English, as do error details from the system.

The translations are in [`locales/`](locales), one JSON file per language, built into the
launcher; anything a translation lacks falls back to English. They're new, so corrections from
native speakers are welcome: edit the file and run `cargo test`, which checks that every file has
the same keys and placeholders as `en.json`.

Chinese, Hindi, Arabic and Bengali need letters the bundled Geist font doesn't have, so the
launcher uses a font already on the computer: Microsoft YaHei, Nirmala UI and Segoe UI on Windows;
PingFang or Hiragino, Kohinoor and SF Arabic or Geeza Pro on macOS; Noto on Linux (Debian/Ubuntu:
`fonts-noto-cjk` and `fonts-noto-core`). If there's none, Settings says so. Arabic text reads right
to left, but the window's layout stays left to right.

The READMEs in other languages are translations of this one, which is the reference.

## The catalog drives everything

The list of apps isn't built into the launcher. It comes from
[`catalog/catalog.json`](catalog/catalog.json) in this repository, which is published at
`https://haruny.github.io/craftcraft/catalog.json` whenever it changes on `main`. Both the
launcher and the website read it. The launcher fetches
it:

- at start-up, sending the last ETag so an unchanged catalog costs one tiny request;
- every few hours while it runs (8 by default; **Settings** sets 1–168), retrying 15 minutes after a failure;
- when you press **Check for updates**.

It keeps the last good copy, so it opens instantly and works offline. When the catalog changes:

| Change in `catalog.json` | What the launcher shows |
|---|---|
| A new app in `apps` | A "new in the catalog" banner and a NEW badge, until the user clicks **Got it** |
| `launcher.version` newer than this build | An update banner with a download link |
| A new entry in `notices` | A banner until dismissed |
| A project publishes a release | **Update** on that app's card |
| An app is listed in `featured.json` | It leads the default Featured order, with a Featured badge |

**Releases come from one file, not from GitHub's API.** Every hour the catalog workflow gathers
each app's latest release (files and SHA-256s), stars and license into
`https://haruny.github.io/craftcraft/releases.json`. The launcher fetches that with an ETag next to
the catalog, so checking every app for updates costs one request, however long the catalog gets,
and nobody runs into GitHub's limit of 60 API requests an hour. Only an app the index doesn't
cover yet (a local catalog being tested, say) is looked up on GitHub directly. Stars and release
dates also drive the **Sort** menu: Featured, Most popular, Name and Recently updated.

**To add an app, send a pull request that adds it to `catalog/catalog.json`**, or point an AI
agent at [ADD_APP.md](ADD_APP.md) with the app's GitHub repository.
[CONTRIBUTING.md](CONTRIBUTING.md) has the rules: the app must replace paid software, and be open
source on github.com.

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
in place. MSI packages run with `/passive`, and with `ALLUSERS=1` when the installed copy is for
everyone, so Windows asks for an administrator's approval; Windows Installer's own records say which
copies those are, since it lists copies installed just for one user under HKLM too. A command-line
app installed some other way (winget, cargo, a script) can't be replaced, so **Install latest**
puts an up-to-date copy in the launcher's folder instead. **Uninstall…** runs the app's own
uninstaller.

When an install, update, uninstall or **Open** fails, the card says which step failed and why, for
example *Installer failed: This app is already installed for everyone…*, with all of it on hover.
For an MSI package the reason comes from Windows Installer's log, which **⋯ → Installer log** opens;
**⋯ → Copy error** copies it all.

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
Linux, or `$CRAFTCRAFT_HOME` when set. Nothing is downloaded until you click **Install** or
**Update**. On Windows, installers are deleted once they've run and an MSI's log once it succeeds;
on macOS and Linux, where the system installer is still open when the launcher hands over, they go
after a week. Debug mode keeps them all.

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

## Releases

Pushing a tag like `v0.2.0` (matching `version` in `Cargo.toml`) runs
[release.yml](.github/workflows/release.yml). It builds Windows (x86_64, Arm), macOS (one universal
`.app`, also as a `.dmg`) and Linux (x86_64, Arm) on their own runners, installs each build with
the install script to test it, packages the signed Windows builds as `.msi` installers and tests
those too, and publishes a GitHub release with the packages and a `SHA256SUMS` file. Running the
workflow by hand does everything except publish: the builds are attached to the run, for testing.
After a release, bump `launcher.version` in the catalog so older launchers offer the update.

To build and try a package locally, the way users get it:

```powershell
./scripts/dist.ps1                                   # Windows: dist/craftcraft-windows-<arch>.zip
$env:CRAFTCRAFT_DIST = "$PWD\dist"; ./install/install.ps1
# The .msi, with WiX 3 (https://github.com/wixtoolset/wix3) installed:
./scripts/msi.ps1 -Exe target/x86_64-pc-windows-msvc/release/craftcraft.exe -Arch x86_64
```
```sh
sh scripts/dist.sh                                   # macOS or Linux
CRAFTCRAFT_DIST="$PWD/dist" sh install/install.sh
```

### Signing

The release workflow signs the builds once these are set in the repository's **Settings → Secrets
and variables → Actions**. Until then, it publishes the builds unsigned.

**macOS (Developer ID, then notarization).** All five are secrets:

| Secret | What it is |
|---|---|
| `APPLE_CERTIFICATE` | A **Developer ID Application** certificate with its private key, exported from Keychain Access as a `.p12`, then base64-encoded (`base64 -i cert.p12 \| pbcopy`) |
| `APPLE_CERTIFICATE_PASSWORD` | The password you gave the `.p12` |
| `APPLE_API_KEY` | An App Store Connect API key (Users and Access → Integrations → Team Keys, role Developer): the `.p8` file, base64-encoded |
| `APPLE_API_KEY_ID` | That key's ID |
| `APPLE_API_ISSUER` | The Issuer ID shown above the keys |

With only the first two, the app is signed but not notarized, and Gatekeeper still warns when it's
downloaded in a browser. To sign locally, set `APPLE_SIGNING_IDENTITY` (and the `APPLE_API_KEY_*`
variables, with `APPLE_API_KEY_PATH` pointing at the `.p8`) before running `scripts/dist.sh`.

**Windows (Microsoft Artifact Signing, formerly Trusted Signing).** No secrets: the workflow signs
in to Azure with GitHub's OpenID Connect, which Azure trusts only from this repository's `release`
environment.

1. In the Azure portal, create an **Artifact Signing account** (the Basic tier is enough) and note
   its endpoint, e.g. `https://eus.codesigning.azure.net/`.
2. In the account, complete **identity validation** (Public), then create a **certificate profile**
   of type Public Trust from it.
3. In Microsoft Entra ID, register an app. Under Certificates & secrets → Federated credentials,
   add one for GitHub Actions: organization `haruny`, repository `craftcraft`, entity type
   **Environment**, name `release`.
4. On the signing account, give that app the **Artifact Signing Certificate Profile Signer** role
   (Access control → Add role assignment).
5. In GitHub, create the environment `release` (Settings → Environments). Limit it to the `main`
   branch and `v*` tags, so only reviewed code is signed.
6. Add these repository variables:

| Variable | Value |
|---|---|
| `AZURE_CLIENT_ID` | The app registration's Application (client) ID |
| `AZURE_TENANT_ID` | Its Directory (tenant) ID |
| `AZURE_SUBSCRIPTION_ID` | The subscription the signing account is in |
| `ARTIFACT_SIGNING_ENDPOINT` | The endpoint from step 1 |
| `ARTIFACT_SIGNING_ACCOUNT` | The signing account's name |
| `ARTIFACT_SIGNING_PROFILE` | The certificate profile's name |

From then on, every run of the release workflow signs `craftcraft.exe` in both Windows zips,
x86_64 and Arm, and checks each signature and timestamp before the zips are published. A run
started by hand signs too, so you can try it before tagging.

## Privacy

The full details are in [Privacy_Statement.md](Privacy_Statement.md). In short:

- **Counts.** Release builds fetch the catalog through CraftCraft's own endpoint, which counts
  catalog checks per day by launcher version, operating system and version, processor and country.
  It keeps no IP address or id. This is how we know which computers CraftCraft runs on.
- **Usage statistics**, which you can turn off. Launches, and which apps you install, update, open
  and remove, sent under a random id that isn't linked to you or your computer. Nothing is sent
  until you've seen the notice about it. Turn it off there or in Settings > Privacy.
- `DO_NOT_TRACK=1` or `CRAFTCRAFT_NO_TELEMETRY=1` turns off both: the catalog then comes
  straight from GitHub Pages.

A build from source has neither (see [telemetry/README.md](telemetry/README.md)). Otherwise
CraftCraft connects only to fetch release information from `haruny.github.io` and `api.github.com`,
the app icons the catalog lists, and the release files you choose to install from `github.com`.
Those services see your IP address, as for any web request.

## Layout

```
catalog/         catalog.json and featured.json; check.mjs (CI runs it on every change); publish.mjs
                 and releases.mjs, which build what's published (catalog.json, releases.json)
locales/         the launcher's text, one JSON file per language
install/         install.ps1 and install.sh, the one-line installers
telemetry/       the usage statistics endpoint (a Cloudflare Worker)
installer/       craftcraft.wxs, the Windows .msi (WiX 3)
scripts/         dist.ps1 and dist.sh, which build and package a release (dist.sh: the .dmg too);
                 msi.ps1, which packages the .msi
src/index.rs     the release index: every app's latest release and stars, in one file
src/catalog.rs   the catalog format: parsing, validation, platforms, picking a release file
src/feed.rs      fetching and caching the catalog
src/github.rs    an app's latest release from GitHub's API, for apps the index doesn't cover
src/detect.rs    finding apps already installed (registry, app bundles, packages, PATH)
src/install.rs   download, verify, unpack, swap in, shims, uninstall
src/system.rs    opening apps, terminals, folders and links; PATH
src/icons.rs     app icons, cached and decoded off the UI thread
src/worker.rs    background jobs and the events they send the UI
src/i18n.rs      the launcher's text in each language (from locales/), and right-to-left lines
src/fonts.rs     finding a system font for Chinese, Hindi, Arabic and Bengali
src/tray.rs      the tray (menu bar) icon on Windows and macOS; on Windows, keeping to one running copy
src/ui.rs        the window
src/theme.rs     colours, fonts and widget styling
```

## Sponsor

CraftCraft is free and built in my spare time. If it saves you a license fee or two, you can
support it through [GitHub Sponsors](https://github.com/sponsors/haruny).

## License

MIT OR Apache-2.0. The bundled Geist fonts (`assets/fonts`) are under the SIL Open Font License
(`assets/fonts/OFL.txt`). Each app the launcher installs is under its own license.
