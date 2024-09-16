#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Detect the operating system
OS="$(uname)"
ARCH="$(uname -m)"

echo -e "${BLUE}Installing Starknet Converter CLI...${NC}"

# Define the GitHub repository and release information
REPO="Leonard-Pat/starknet-converter-cli"
VERSION="latest"  # You can change this to a specific version if needed

# Determine the correct binary for the OS and architecture
case "$OS" in
    "Darwin")
        case "$ARCH" in
            "x86_64") BINARY="snconvert-macos-x86_64" ;;
            "arm64") BINARY="snconvert-macos-arm64" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}" && exit 1 ;;
        esac
        ;;
    "Linux")
        case "$ARCH" in
            "x86_64") BINARY="snconvert-linux-x86_64" ;;
            "aarch64") BINARY="snconvert-linux-aarch64" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}" && exit 1 ;;
        esac
        ;;
    *)
        echo -e "${RED}Unsupported operating system: $OS${NC}" && exit 1
        ;;
esac

# Create a temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download the latest release
echo -e "${BLUE}Downloading Starknet Converter CLI...${NC}"
curl -sLO "https://github.com/$REPO/releases/download/$VERSION/$BINARY"

# Make the binary executable
chmod +x "$BINARY"

# Move the binary to a directory in the user's PATH
TARGET_DIR="/usr/local/bin"
if [ ! -w "$TARGET_DIR" ]; then
    TARGET_DIR="$HOME/.local/bin"
    mkdir -p "$TARGET_DIR"
fi

mv "$BINARY" "$TARGET_DIR/snconvert"

echo -e "${GREEN}Starknet Converter CLI has been installed to $TARGET_DIR/snconvert${NC}"

# Clean up
cd
rm -rf "$TMP_DIR"

echo -e "${GREEN}Installation complete!${NC}"
echo -e "${BLUE}You can now use the Starknet Converter CLI by running 'snconvert' in your terminal.${NC}"

# Check if the installation directory is in PATH
if [[ ":$PATH:" != *":$TARGET_DIR:"* ]]; then
    echo -e "${RED}Warning: $TARGET_DIR is not in your PATH.${NC}"
    echo -e "${BLUE}Add the following line to your shell configuration file (.bashrc, .zshrc, etc.):${NC}"
    echo -e "${GREEN}export PATH=\"\$PATH:$TARGET_DIR\"${NC}"
fi
