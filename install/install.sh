#!/bin/sh
# Installs or updates CraftCraft for the current user:
#
#   curl -fsSL https://haruny.github.io/craftcraft/install.sh | sh
#
# Downloads the latest release from https://github.com/haruny/craftcraft/releases, checks its
# SHA-256, installs it and opens it. Run it again to update.
#
#   macOS  ~/Applications/CraftCraft.app
#   Linux  ~/.local/bin/craftcraft, with a menu entry
#
#   CRAFTCRAFT_DIST       install a build from this folder or address instead (for testing)
#   CRAFTCRAFT_NO_LAUNCH  1: don't open CraftCraft afterwards

set -eu

REPO=haruny/craftcraft

say() { printf '%s\n' "$*"; }
fail() { printf 'craftcraft install: %s\n' "$*" >&2; exit 1; }
need() { command -v "$1" >/dev/null 2>&1 || fail "needs $1, which isn't installed"; }

fetch() { # fetch <file> <dest>
    case "$dist" in
    http://* | https://*) curl -fsSL --proto '=https' --tlsv1.2 -o "$2" "$dist/$1" || fail "couldn't download $dist/$1" ;;
    *) cp "$dist/$1" "$2" || fail "no $1 in $dist" ;;
    esac
}

sha256() {
    if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1"; else shasum -a 256 "$1"; fi | cut -d ' ' -f 1
}

main() {
    need curl
    need tar

    case "$(uname -s)" in
    Darwin) os=macos package=craftcraft-macos-universal.tar.gz ;;
    Linux)
        os=linux
        case "$(uname -m)" in
        x86_64 | amd64) package=craftcraft-linux-x86_64.tar.gz ;;
        aarch64 | arm64) package=craftcraft-linux-aarch64.tar.gz ;;
        *) fail "there's no CraftCraft build for $(uname -m) yet" ;;
        esac
        ;;
    *) fail "this script is for macOS and Linux; on Windows run: irm https://haruny.github.io/craftcraft/install.ps1 | iex" ;;
    esac

    dist=${CRAFTCRAFT_DIST:-}
    if [ -z "$dist" ]; then
        # github.com/<repo>/releases/latest redirects to .../releases/tag/<tag>.
        latest=$(curl -fsSLI -o /dev/null -w '%{url_effective}' "https://github.com/$REPO/releases/latest") ||
            fail "couldn't reach https://github.com/$REPO/releases"
        tag=${latest##*/tag/}
        [ "$tag" != "$latest" ] || fail "no CraftCraft release has been published yet"
        dist="https://github.com/$REPO/releases/download/$tag"
    fi

    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"' EXIT INT TERM

    fetch "$package" "$tmp/$package"
    fetch SHA256SUMS "$tmp/SHA256SUMS"
    expected=$(awk -v f="$package" '$2 == f || $2 == "*" f { print $1; exit }' "$tmp/SHA256SUMS")
    [ -n "$expected" ] || fail "SHA256SUMS has no entry for $package"
    actual=$(sha256 "$tmp/$package")
    [ "$actual" = "$expected" ] || fail "$package is damaged: its SHA-256 is $actual, not $expected. Nothing was installed."

    mkdir "$tmp/unpacked"
    tar -xzf "$tmp/$package" -C "$tmp/unpacked"

    if [ "$os" = macos ]; then
        apps="$HOME/Applications"
        app="$apps/CraftCraft.app"
        exe="$app/Contents/MacOS/craftcraft"
        # Close a running copy, so opening it afterwards starts the new version.
        pkill -f "^$exe" 2>/dev/null && sleep 1 || true
        mkdir -p "$apps"
        rm -rf "$app.new"
        mv "$tmp/unpacked/CraftCraft.app" "$app.new"
        rm -rf "$app"
        mv "$app.new" "$app"
    else
        bin="$HOME/.local/bin"
        exe="$bin/craftcraft"
        data=${XDG_DATA_HOME:-$HOME/.local/share}
        mkdir -p "$bin" "$data/applications" "$data/icons/hicolor/scalable/apps"
        pkill -f "^$exe" 2>/dev/null && sleep 1 || true
        # Replace by renaming, which works even while the old binary is running.
        install -m 755 "$tmp/unpacked/craftcraft" "$exe.new"
        mv -f "$exe.new" "$exe"
        install -m 644 "$tmp/unpacked/craftcraft.svg" "$data/icons/hicolor/scalable/apps/craftcraft.svg"
        cat >"$data/applications/craftcraft.desktop" <<EOF
[Desktop Entry]
Type=Application
Name=CraftCraft
Comment=Install, update and open software rewritten in Rust
Exec=$exe
Icon=craftcraft
Categories=Utility;
Terminal=false
EOF
        command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database -q "$data/applications" 2>/dev/null || true
    fi

    version=$("$exe" --version | sed 's/^craftcraft //') || fail "the installed CraftCraft doesn't run"

    if [ "$os" = macos ]; then
        say "CraftCraft $version is installed in $app."
        say "Open it from Launchpad or Spotlight. Run the same command again to update it."
        [ "${CRAFTCRAFT_NO_LAUNCH:-}" = 1 ] || open "$app"
    else
        say "CraftCraft $version is installed in $exe."
        say "Open it from your applications menu, or run craftcraft. Run the same command again to update it."
        case ":$PATH:" in
        *":$bin:"*) ;;
        *) say "Note: $bin isn't on your PATH, so the craftcraft command won't be found until you add it." ;;
        esac
        if [ "${CRAFTCRAFT_NO_LAUNCH:-}" != 1 ] && [ -n "${DISPLAY:-}${WAYLAND_DISPLAY:-}" ]; then
            nohup "$exe" >/dev/null 2>&1 &
        fi
    fi
}

# Everything runs from here, so a download cut off halfway can't run half a script.
main "$@"
