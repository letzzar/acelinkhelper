#!/usr/bin/env bash
#
# Builds a universal (arm64 + x86_64) AcelinkHelper.app and packages it as a DMG.
#
# Usage: ./scripts/build-macos-dmg.sh [version]
#        version defaults to the one in Cargo.toml
#
set -euo pipefail

cd "$(dirname "$0")/.."

VERSION="${1:-$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)}"
APP_NAME="AcelinkHelper"
BUNDLE_ID="com.acelinkhelper.app"
DIST="dist"
APP="$DIST/$APP_NAME.app"
DMG="$DIST/$APP_NAME-$VERSION-universal.dmg"

echo "==> Building AcelinkHelper $VERSION (universal)"

rustup target add aarch64-apple-darwin x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

rm -rf "$DIST"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

echo "==> Merging architectures with lipo"
lipo -create -output "$APP/Contents/MacOS/acelinkhelper" \
  "target/aarch64-apple-darwin/release/acelinkhelper" \
  "target/x86_64-apple-darwin/release/acelinkhelper"
lipo -info "$APP/Contents/MacOS/acelinkhelper"

echo "==> Generating app.icns"
rm -rf "$DIST/$APP_NAME.iconset"
cp -R iconset "$DIST/$APP_NAME.iconset"
iconutil -c icns "$DIST/$APP_NAME.iconset" -o "$APP/Contents/Resources/app.icns"
rm -rf "$DIST/$APP_NAME.iconset"

echo "==> Writing Info.plist"
cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleDevelopmentRegion</key>
	<string>en</string>
	<key>CFBundleExecutable</key>
	<string>acelinkhelper</string>
	<key>CFBundleIdentifier</key>
	<string>$BUNDLE_ID</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>$APP_NAME</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>$VERSION</string>
	<key>CFBundleVersion</key>
	<string>$VERSION</string>
	<key>CFBundleIconFile</key>
	<string>app.icns</string>
	<key>LSMinimumSystemVersion</key>
	<string>10.13</string>
	<key>NSHighResolutionCapable</key>
	<true/>
	<key>CFBundleURLTypes</key>
	<array>
		<dict>
			<key>CFBundleURLName</key>
			<string>Acestream Protocol Handler</string>
			<key>CFBundleURLSchemes</key>
			<array>
				<string>acestream</string>
			</array>
		</dict>
	</array>
	<key>LSApplicationCategoryType</key>
	<string>public.app-category.utilities</string>
	<key>NSAppTransportSecurity</key>
	<dict>
		<key>NSAllowsArbitraryLoads</key>
		<true/>
	</dict>
</dict>
</plist>
PLIST

printf 'APPL????' > "$APP/Contents/PkgInfo"

# lipo strips signatures; arm64 macOS refuses to run an unsigned binary,
# so re-sign ad-hoc. This is not notarized - see README for the Gatekeeper note.
echo "==> Ad-hoc signing"
codesign --force --deep --sign - "$APP"
codesign --verify --verbose "$APP"

echo "==> Building DMG"
STAGE="$DIST/stage"
mkdir -p "$STAGE"
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
hdiutil create -volname "$APP_NAME $VERSION" -srcfolder "$STAGE" -ov -format UDZO "$DMG"
rm -rf "$STAGE"

echo "==> Done: $DMG"
