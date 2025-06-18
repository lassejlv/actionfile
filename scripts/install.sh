#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print with color
print() {
    echo -e "${2}${1}${NC}"
}

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if curl is installed
if ! command_exists curl; then
    print "curl is required but not installed. Please install curl first." $RED
    exit 1
fi

# Check if running on macOS
if [ "$(uname -s)" != "Darwin" ]; then
    print "Warn: This installer currently only supports macOS." $YELLOW
    print "Linux support coming soon!" $YELLOW
fi

# Get the latest release version from GitHub API
print "Fetching latest release information..." $YELLOW
API_URL="https://api.github.com/repos/lassejlv/actionfile/releases/latest"
LATEST_VERSION=$(curl -s $API_URL | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    print "Failed to fetch latest version information." $RED
    exit 1
fi

print "Latest version: $LATEST_VERSION" $GREEN

# Determine architecture
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64)
        ARCH="amd64"
        ;;
    arm64)
        ARCH="arm64"
        ;;
    *)
        print "Unsupported architecture: $ARCH" $RED
        exit 1
        ;;
esac

# Construct filename
FILENAME="actionfile-darwin-${ARCH}-${LATEST_VERSION}"

# Construct download URL
DOWNLOAD_URL="https://github.com/lassejlv/actionfile/releases/download/${LATEST_VERSION}/${FILENAME}"

# Create bin directory if it doesn't exist
BIN_DIR="$HOME/.local/bin"
mkdir -p "$BIN_DIR"

# Download the binary
print "Downloading actionfile..." $YELLOW
curl -L -o "$BIN_DIR/actionfile" "$DOWNLOAD_URL"

if [ $? -ne 0 ]; then
    print "Failed to download actionfile" $RED
    exit 1
fi

# Make the binary executable
chmod +x "$BIN_DIR/actionfile"

# Check if $BIN_DIR is in PATH
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    print "\nIMPORTANT: Add the following line to your shell configuration file (.zshrc, .bashrc, etc.):" $YELLOW
    print "export PATH=\"\$PATH:$BIN_DIR\"" $YELLOW
    print "\nThen restart your terminal or run: source ~/.zshrc (or your shell's config file)" $YELLOW
fi

print "\nactionfile has been successfully installed!" $GREEN
print "You can now use it by running: actionfile" $GREEN
