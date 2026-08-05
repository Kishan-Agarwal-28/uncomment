#!/usr/bin/env bash
set -e

# Repository information
REPO="Kishan-Agarwal-28/uncomment"
BIN_NAME="uncomment"

# Function to detect the operating system
get_os() {
    case "$(uname -s)" in
        Linux*)     echo "linux";;
        Darwin*)    echo "macos";;
        *)          echo "unsupported";;
    esac
}

# Function to detect the architecture
get_arch() {
    case "$(uname -m)" in
        x86_64|amd64) echo "x86_64";;
        arm64|aarch64) echo "aarch64";;
        *)             echo "unsupported";;
    esac
}

OS=$(get_os)
ARCH=$(get_arch)

if [ "$OS" = "unsupported" ] || [ "$ARCH" = "unsupported" ]; then
    echo "Error: Unsupported operating system or architecture ($OS $ARCH)."
    exit 1
fi

# Determine the target triple
if [ "$OS" = "linux" ]; then
    TARGET="${ARCH}-unknown-linux-gnu"
elif [ "$OS" = "macos" ]; then
    TARGET="${ARCH}-apple-darwin"
fi

# Fetch the latest release version
echo "Fetching latest release information for $REPO..."
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest")
VERSION=$(echo "$LATEST_RELEASE" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$VERSION" ]; then
    echo "Error: Could not determine latest version. Check your internet connection or API rate limits."
    exit 1
fi

echo "Installing $BIN_NAME version $VERSION for $TARGET"

# Download the release archive
FILENAME="${BIN_NAME}-${VERSION}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$VERSION/$FILENAME"
TMP_DIR=$(mktemp -d)

echo "Downloading $DOWNLOAD_URL..."
curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$FILENAME"

# Extract the archive
echo "Extracting..."
tar -xzf "$TMP_DIR/$FILENAME" -C "$TMP_DIR"

# Install the binary
INSTALL_DIR="/usr/local/bin"
if [ ! -w "$INSTALL_DIR" ]; then
    echo "Installing to ~/.local/bin since $INSTALL_DIR is not writable..."
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$BIN_NAME"

# Clean up
rm -rf "$TMP_DIR"

echo ""
echo "$BIN_NAME was successfully installed to $INSTALL_DIR/$BIN_NAME"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Warning: $INSTALL_DIR is not in your PATH."
    echo "Add it to your PATH by adding the following line to your shell configuration:"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
fi
