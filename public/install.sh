#!/usr/bin/env bash
set -e

# Terminal Helper (th) Installation Script

echo "================================================="
echo "   Installing Terminal Helper (th) "
echo "================================================="

REPO="tnmyk/th"

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     TARGET_OS="linux";;
    Darwin*)    TARGET_OS="macos";;
    *)          echo "❌ Unsupported OS: ${OS}" && exit 1;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64*)    TARGET_ARCH="x86_64";;
    arm64*|aarch64*) TARGET_ARCH="aarch64";;
    *)          echo "❌ Unsupported architecture: ${ARCH}" && exit 1;;
esac

TARGET="${TARGET_ARCH}-${TARGET_OS}"
echo "Detected platform: ${TARGET}"

BASE_URL="https://th.tnmyk.workers.dev"
LATEST_URL="${BASE_URL}/th-${TARGET}.tar.gz"
TMP_DIR=$(mktemp -d)

echo "Downloading latest release..."
if ! curl -fsSL "$LATEST_URL" -o "$TMP_DIR/th.tar.gz"; then
    echo "❌ Error: Failed to download from $LATEST_URL"
    echo "Make sure the release exists on GitHub for your platform."
    exit 1
fi

echo "Extracting..."
tar -xzf "$TMP_DIR/th.tar.gz" -C "$TMP_DIR"

INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Installing to $INSTALL_DIR..."
mv "$TMP_DIR/th" "$INSTALL_DIR/th"
chmod +x "$INSTALL_DIR/th"
rm -rf "$TMP_DIR"

echo ""
echo "✅ Installation complete!"
echo "Make sure $INSTALL_DIR is in your PATH."
echo "If you can't run 'th', add this to your ~/.zshrc or ~/.bashrc:"
echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
echo ""
echo "To get started, run:"
echo "  th --setup"
echo "================================================="
