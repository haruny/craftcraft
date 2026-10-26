# Contributing

## Add an app

Every app in CraftCraft comes from one file, [`catalog/catalog.json`](catalog/catalog.json). To add
one, open a pull request that adds an entry to it. Once it's merged, every launcher and the
[website](https://haruny.github.io/craftcraft-site/) pick it up within a few hours, with no new
launcher release.

**The quick way:** point your AI agent (Claude Code, Codex, Cursor…) at
[ADD_APP.md](ADD_APP.md) and give it the app's GitHub repository:

> Follow https://github.com/haruny/craftcraft/blob/main/ADD_APP.md to add github.com/OWNER/NAME to CraftCraft.

It checks the app qualifies, works out the entry from the repository and its latest release,
checks it, and opens the pull request. To do it by hand, follow the same file: it's also the
reference for every field.

### What qualifies

1. **It replaces paid software.** CraftCraft is for the Rust alternatives to software people pay
   for: Photoshop, Office, AutoCAD, SolidWorks, TeamViewer. A rewrite of something already free,
   like grep or VS Code, doesn't fit, however good it is.
2. **It's open source, with its source on github.com.** The repository must be public and have an
   open-source license file.
3. **It publishes ready-to-run builds** as files on its GitHub Releases. The launcher installs
   only from there, and checks each file against the SHA-256 GitHub shows for it.

The check that runs on every pull request enforces rule 2 and the builds. A maintainer reviews
rule 1, and installs the app before merging.

### An entry

Seven fields, plus `launch` when the download is an installer:

```json
{
  "id": "rustdesk",
  "name": "RustDesk",
  "replaces": ["TeamViewer", "AnyDesk"],
  "category": "utilities",
  "tagline": "Remote desktop you can self-host",
  "repo": "rustdesk/rustdesk",
  "assets": {
    "windows-x86_64": "rustdesk-*-x86_64.msi",
    "macos-aarch64": "rustdesk-*-aarch64.dmg",
    "linux-x86_64": "rustdesk-*-x86_64.AppImage"
  },
  "launch": {
    "windows": "%ProgramFiles%\\RustDesk\\rustdesk.exe",
    "macos": "/Applications/RustDesk.app"
  }
}
```

Each `assets` pattern matches exactly one file in the latest release, with `*` in place of the
version. The license, star count and latest version come from GitHub, so they aren't in the entry.
Your editor autocompletes and checks the entry, because the file points at
[`catalog.schema.json`](catalog/catalog.schema.json).

### Check it

```sh
node catalog/check.mjs --online   # needs Node 18+; GITHUB_TOKEN=… raises GitHub's rate limit
```

To see the entry in the launcher before it's merged, run a debug build against your copy:

```sh
CRAFTCRAFT_FEED=catalog/catalog.json CRAFTCRAFT_HOME=/tmp/craftcraft-test cargo run
```

On Windows PowerShell: `$env:CRAFTCRAFT_FEED = "catalog\catalog.json"`, then `cargo run`.

If a project renames its release files later, the daily check fails and we'll ask for the pattern
to be updated.

## For maintainers

| File | What it's for |
|---|---|
| `catalog/catalog.json` | The apps, categories, notices and launcher version. Contributors' pull requests change it |
| `catalog/featured.json` | **Yours alone** (CODEOWNERS): the apps to feature, in order. They lead the Featured order in the launcher and on the website, with a Featured badge |
| `catalog/publish.mjs` | Builds the published catalog: merges `featured.json` in, and fills in `added` and `color` where entries leave them out |
| `catalog/releases.mjs` | Builds `releases.json`: every app's latest release (files and SHA-256s), stars and license |
| `catalog/check.mjs` | The check run on pull requests and daily |

Merging to `main` publishes to `https://haruny.github.io/craftcraft/`. `releases.json` is also
rebuilt every hour, so launchers learn about new releases from that one file instead of each
asking GitHub about every app.

For CODEOWNERS to apply, turn on **Require review from Code Owners** in the `main` branch's
protection rules.

Other fields in `catalog.json`:

- `refresh_hours` (1–168): how often launchers re-check the catalog.
- `launcher.version`: the newest launcher release. Launchers older than this show an update banner,
  so bump it after publishing a launcher release.
- `notices`: announcements shown in the launcher until the user dismisses them:
  `{ "id": "hello", "title": "…", "body": "…", "url": "https://…" }`. Give each one a new `id`.
- `categories`: the sidebar sections, in order.
- `updated`: bump it when you merge a change; the website shows it.
- `schema`: the format version. Launchers refuse a catalog with a newer schema, so change it only
  for breaking changes, and only after a launcher that understands the new format has shipped.

## Changing the launcher

Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` before opening a pull
request; CI runs the same on Windows, macOS and Linux.
