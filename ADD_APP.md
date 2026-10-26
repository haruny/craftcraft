# Add an app to CraftCraft: instructions for an AI agent

You are helping someone add an app to the CraftCraft catalog, `catalog/catalog.json` in
https://github.com/haruny/craftcraft. The only input you need is the app's GitHub repository
(`owner/name`). Work out everything else yourself, from the GitHub API and the repository. Ask the
person only when these steps say to.

The result is one JSON object to add to the end of the `apps` array, then a pull request.

## 1. Check that the app qualifies

Use the GitHub API (`gh api repos/OWNER/NAME`, or `https://api.github.com/repos/OWNER/NAME`).
Stop and explain why if any of these fail:

1. **It replaces paid software.** Name the commercial product(s) it's an alternative to (for
   example Photoshop, AutoCAD, TeamViewer). Read the README to find them. If it only replaces
   something already free (grep, VS Code, a free terminal), it doesn't qualify.
2. **It's open source on github.com.** The repository is public and `license.spdx_id` is set
   (not `null`).
3. **It's written in Rust.** `GET repos/OWNER/NAME/languages` lists Rust as the main language.
4. **It has ready-to-run builds.** `GET repos/OWNER/NAME/releases/latest` exists and has files
   (`assets`) for at least one of Windows, macOS or Linux.
5. **It isn't listed already.** No entry in `catalog/catalog.json` has the same `repo`.

## 2. Write the entry

Required fields:

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

- **`id`**: the name in lowercase, with letters, digits and `-` only. It must be unique in the
  catalog.
- **`name`**: the product name exactly as its installer registers it, which is usually the name
  in the README title. The launcher matches this against installed programs, so on macOS it must
  equal the `.app` bundle name.
- **`replaces`**: the paid products from step 1, best known first.
- **`category`**: one of the `id`s in the catalog's `categories` list. Read the list from the
  file, don't guess it.
- **`tagline`**: what it does, in at most 6 words, without the app's name and with no full stop.
  For example "Image editing that feels familiar".
- **`repo`**: `owner/name`, as GitHub spells it.
- **`assets`**: for each platform, a pattern matching **exactly one** file in the latest release.
  - Replace the version number in the file name with `*`. For example, `myapp-1.4.2-x64.msi`
    becomes `myapp-*-x64.msi`. This keeps the pattern working for future releases.
  - Platform keys are `windows-x86_64`, `windows-aarch64`, `macos-x86_64`, `macos-aarch64`,
    `linux-x86_64` and `linux-aarch64`. Include only the ones a file exists for. A universal
    macOS file can be used for both `macos-` keys.
  - Prefer the project's own installer: `.msi` (else a setup `.exe`) on Windows, `.dmg` on macOS,
    `.AppImage` (else `.deb`) on Linux. Fall back to a `.zip` or `.tar.gz`.
  - Skip debug builds, symbols, source archives, checksums and signatures.
- **`launch`** (when an asset is an installer, `.msi`, `.exe`, `.dmg`, `.pkg`, `.deb` or `.rpm`):
  where the installed app ends up, so the launcher can open it and detect existing installs. On
  Windows that's usually `%ProgramFiles%\\<Name>\\<exe>.exe`, and on macOS `/Applications/<Name>.app`.
  Find the exact path in the project's installer config (WiX `.wxs`, `tauri.conf.json`,
  `Cargo.toml` `[package.metadata.bundle]`, `cargo-dist` or `packager` settings) or its docs. If
  you can't confirm it, give your best guess and say so in the pull request.
- **`bin`** (only when an asset is a `.zip` or `.tar.gz`): the executable's file name inside the
  archive, e.g. `["myapp"]`. Check the archive's contents.

Optional fields. Add them only when you have the information:

- `description`: one or two plain sentences on what it does.
- `homepage`: the project's website, `https://` only. Leave it out if it's just the GitHub page.
- `icon`: a square PNG or JPEG of the app's icon, as a `https://raw.githubusercontent.com/...`
  URL from the repository, 128–512 px.
- `kind`: leave it out for desktop apps. Use `"tui"` for apps that run full-screen in a terminal,
  `"cli"` for command-line tools.

**Don't add** `license`, `color` or `added`: the license comes from GitHub, and the colour and
date are filled in when the catalog is published. **Never** edit `featured`, `featured.json`,
other apps' entries or anything outside `catalog/catalog.json`.

## 3. Check it

From a clone of the repository:

```sh
node catalog/check.mjs --online
```

It must report 0 errors. Fix whatever it reports and run it again. Setting `GITHUB_TOKEN` (for
example `GITHUB_TOKEN=$(gh auth token)`) avoids GitHub's rate limit.

## 4. Open the pull request

Commit only the change to `catalog/catalog.json`, on a new branch of a fork, and open a pull
request to `haruny/craftcraft`. Title it `Add <name>`. In the description, fill in the
checklist from `.github/pull_request_template.md`. State which paid products it replaces, and
anything you guessed (most often the `launch` path).

If you can't open pull requests, give the person the JSON entry and these instructions instead.
