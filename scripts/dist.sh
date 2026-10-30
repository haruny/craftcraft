#!/bin/sh
# Builds CraftCraft for macOS or Linux and packages it as the release does, plus dist/SHA256SUMS:
#
#   macOS  dist/craftcraft-macos-universal.tar.gz   CraftCraft.app, for Apple silicon and Intel
#          dist/craftcraft-macos-universal.dmg      the same, as a disk image
#   Linux  dist/craftcraft-linux-<arch>.tar.gz      the craftcraft binary and its icon
#
#   scripts/dist.sh             this computer's platform
#   scripts/dist.sh <target>    a specific Linux target, e.g. aarch64-unknown-linux-gnu
#
# To try the result the way a user would: CRAFTCRAFT_DIST="$PWD/dist" sh install/install.sh
#
# On macOS, APPLE_SIGNING_IDENTITY ("Developer ID Application: …") signs the app with that
# certificate instead of ad hoc, and APPLE_API_KEY_PATH, APPLE_API_KEY_ID and APPLE_API_ISSUER (an
# App Store Connect API key) notarize it. Both apply to the disk image too.

set -eu
cd "$(dirname "$0")/.."

version=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)
mkdir -p dist

case "$(uname -s)" in
Darwin)
    # One universal binary, so a single download works on every Mac.
    for target in aarch64-apple-darwin x86_64-apple-darwin; do
        rustup target add "$target" >/dev/null 2>&1 || true
        cargo build --release --locked --target "$target"
    done
    app=target/dist/CraftCraft.app
    rm -rf target/dist
    mkdir -p "$app/Contents/MacOS"
    lipo -create -output "$app/Contents/MacOS/craftcraft" \
        target/aarch64-apple-darwin/release/craftcraft target/x86_64-apple-darwin/release/craftcraft
    cat >"$app/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>CraftCraft</string>
  <key>CFBundleDisplayName</key><string>CraftCraft</string>
  <key>CFBundleIdentifier</key><string>io.CraftCraft.CraftCraft</string>
  <key>CFBundleExecutable</key><string>craftcraft</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>$version</string>
  <key>CFBundleVersion</key><string>$version</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
EOF
    if [ -n "${APPLE_SIGNING_IDENTITY:-}" ]; then
        # Developer ID, with the hardened runtime notarization requires.
        codesign --force --options runtime --timestamp --sign "$APPLE_SIGNING_IDENTITY" "$app"
        codesign --verify --strict --verbose=2 "$app"
    else
        # Apple silicon only runs signed code; an ad-hoc signature is enough when nothing quarantines it.
        codesign --force --sign - "$app"
    fi
    if [ -n "${APPLE_API_KEY_PATH:-}" ]; then
        # Notarize, then staple the ticket so Gatekeeper accepts the app offline too.
        ditto -c -k --keepParent "$app" target/dist/notarize.zip
        xcrun notarytool submit target/dist/notarize.zip --wait \
            --key "$APPLE_API_KEY_PATH" --key-id "$APPLE_API_KEY_ID" --issuer "$APPLE_API_ISSUER"
        xcrun stapler staple "$app"
        rm target/dist/notarize.zip
    fi
    package=craftcraft-macos-universal.tar.gz
    tar -czf "dist/$package" -C target/dist CraftCraft.app

    # The disk image, for downloading in a browser: the app beside a link to Applications.
    dmg=dist/craftcraft-macos-universal.dmg
    mkdir target/dist/dmg
    cp -R "$app" target/dist/dmg/
    ln -s /Applications target/dist/dmg/Applications
    rm -f "$dmg"
    # hdiutil now and then fails with "Resource busy" on a busy machine; a moment later it doesn't.
    for attempt in 1 2 3; do
        hdiutil create -volname CraftCraft -srcfolder target/dist/dmg -fs HFS+ -format UDZO "$dmg" && break
        [ "$attempt" = 3 ] && exit 1
        sleep 5
    done
    rm -rf target/dist/dmg
    if [ -n "${APPLE_SIGNING_IDENTITY:-}" ]; then
        codesign --force --timestamp --sign "$APPLE_SIGNING_IDENTITY" "$dmg"
    fi
    if [ -n "${APPLE_API_KEY_PATH:-}" ]; then
        xcrun notarytool submit "$dmg" --wait \
            --key "$APPLE_API_KEY_PATH" --key-id "$APPLE_API_KEY_ID" --issuer "$APPLE_API_ISSUER"
        xcrun stapler staple "$dmg"
    fi
    ;;
Linux)
    target=${1:-$(rustc -vV | sed -n 's/^host: //p')}
    case "$target" in
    x86_64-*-linux-*) arch=x86_64 ;;
    aarch64-*-linux-*) arch=aarch64 ;;
    *) echo "not a Linux target: $target" >&2; exit 1 ;;
    esac
    cargo build --release --locked --target "$target"
    rm -rf target/dist
    mkdir -p target/dist
    cp "target/$target/release/craftcraft" assets/icon.svg target/dist/
    mv target/dist/icon.svg target/dist/craftcraft.svg
    package=craftcraft-linux-$arch.tar.gz
    tar -czf "dist/$package" -C target/dist craftcraft craftcraft.svg
    ;;
*)
    echo "Use scripts/dist.ps1 on Windows." >&2
    exit 1
    ;;
esac

# One "<sha256>  <file>" line per package.
(cd dist && for f in craftcraft-*; do
    if command -v sha256sum >/dev/null; then sha256sum "$f"; else shasum -a 256 "$f"; fi
done) >dist/SHA256SUMS

echo "Built dist/$package"
