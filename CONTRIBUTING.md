# Contributing

## Add your app to the catalog

Every app in CraftCraft comes from one file, [`catalog/catalog.json`](catalog/catalog.json). To add
yours, open a pull request that adds an entry to it. Once it's merged, every launcher and the
[website](https://haruny.github.io/craftcraft-site/) pick it up within a few hours, with no new
launcher release.

An app fits the catalog when it:

- is a Rust rewrite of, or a Rust alternative to, software people already know;
- is open source, on GitHub;
- publishes ready-to-run builds as files on its **GitHub Releases** (the launcher installs only
  from there, and checks each file against the SHA-256 GitHub shows for it);
- works: people should be able to install it and use it for the thing it replaces.

### 1. Add an entry

Add your app to the end of `apps` in `catalog/catalog.json`:

```jsonc
{
  "id": "ripgrep",                    // lowercase letters, digits and -; unique; also the install folder name
  "name": "ripgrep",
  "replaces": ["grep"],               // what it's a rewrite of
  "category": "cli",                  // an id from "categories" at the top of the file
  "kind": "cli",                      // gui: opens a window · tui: runs in a terminal · cli: a command
  "tagline": "Search file contents, very fast",
  "description": "…",                 // optional, a sentence or two
  "repo": "BurntSushi/ripgrep",       // GitHub owner/name; installs come from its latest release
  "homepage": "https://…",            // optional
  "license": "MIT OR Unlicense",
  "color": "#e11d48",                 // tile colour, used when there's no icon
  "icon": "https://…/icon-128.png",   // optional; a square PNG or JPEG over https, 2 MB at most
  "added": "2026-10-26",              // today; apps added in the last 30 days are marked New
  "assets": {                         // which release file to use on each platform; * matches anything
    "windows-x86_64": "ripgrep-*-x86_64-pc-windows-msvc.zip",
    "macos-aarch64": "ripgrep-*-aarch64-apple-darwin.tar.gz",
    "linux-x86_64": "ripgrep-*-x86_64-unknown-linux-musl.tar.gz"
  },
  "bin": ["rg"]                       // executables inside the archive
}
```

Platforms are `windows-x86_64`, `windows-aarch64`, `macos-x86_64`, `macos-aarch64`,
`linux-x86_64` and `linux-aarch64`. Leave out any you don't build for. Arm Windows and Arm Macs fall
back to the x86_64 file when there's no Arm one.

Use `*` for the version in file names (`myapp-*-linux.tar.gz`), so the entry keeps working when you
publish a new release. Each pattern must match exactly one file in your latest release.

How the launcher installs a file depends on its extension:

| File | Method | What happens |
|---|---|---|
| `.zip`, `.tar.gz`, `.tgz`, `.tar.xz` | `archive` | Unpacked into the launcher's own apps folder. `bin` names the executables, and the first one is what **Open** runs. Command-line tools get a shim in the launcher's `bin` folder |
| `.msi`, `.exe`, `.dmg`, `.pkg`, `.deb`, `.rpm` | `installer` | Downloaded, verified and handed to the system installer. Add `launch` with the installed path per OS (`%LOCALAPPDATA%`, `%ProgramFiles%` and `~` are expanded) so the launcher can find and open it |
| `.AppImage` | `portable` | Kept as a single executable file |

To override the guess, use an object: `{ "file": "Alacritty-*-portable.exe", "method": "portable" }`.

**Desktop apps:** prefer your own installer (`.msi` on Windows, `.dmg` on macOS) and give its
`launch` path:

```jsonc
"launch": {
  "windows": "%ProgramFiles%\\PhotoCraft\\photocraft.exe",
  "macos": "/Applications/PhotoCraft.app"
}
```

The launcher then recognises copies people already installed, and updates upgrade them in place
instead of adding a second copy. It finds installed apps by `name`, as Windows lists installed
programs and as the macOS bundle is called (`/Applications/<name>.app`), so `name` should match
what your installer registers.

Your editor will autocomplete and check the entry: the file points at
[`catalog.schema.json`](catalog/catalog.schema.json).

### 2. Check it

```sh
node catalog/check.mjs --online   # needs Node 18+; GITHUB_TOKEN=… raises GitHub's rate limit
```

`--online` checks that every pattern matches exactly one file in your current release, and warns
when GitHub has no SHA-256 for a file (the launcher then installs it unverified, and says so). The
same check runs on your pull request.

To see your entry in the launcher before it's merged, run a debug build against your copy:

```sh
CRAFTCRAFT_FEED=catalog/catalog.json CRAFTCRAFT_HOME=/tmp/craftcraft-test cargo run
```

On Windows PowerShell: `$env:CRAFTCRAFT_FEED = "catalog\catalog.json"`, then `cargo run`.

### 3. Open the pull request

Change only your entry, and fill in the checklist in the pull request description. A maintainer
installs the app before merging, because the launcher installs whatever the catalog says.

If your release files are renamed later, the daily check fails and we'll ask you to update the
pattern, so please keep watching the repository's notifications for your entry.

## For maintainers: the rest of catalog.json

- `refresh_hours` (1–168): how often launchers re-check the catalog.
- `launcher.version`: the newest launcher release. Launchers older than this show an update banner,
  so bump it after publishing a launcher release.
- `notices`: announcements shown in the launcher until the user dismisses them:
  `{ "id": "hello", "title": "…", "body": "…", "url": "https://…" }`. Give each one a new `id`.
- `categories`: the sidebar sections, in order.
- `updated`: bump it when you merge a change; the website shows it.
- `schema`: the format version. Launchers refuse a catalog with a newer schema, so change it only
  for breaking changes, and only after a launcher that understands the new format has shipped.

Merging to `main` runs the check and publishes the catalog to
`https://haruny.github.io/craftcraft/catalog.json`.

## Changing the launcher

Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` before opening a pull
request; CI runs the same on Windows, macOS and Linux.
