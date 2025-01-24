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

# Binary location
BIN_DIR="$HOME/.local/bin"
BINARY_PATH="$BIN_DIR/actionfile"

# Check if the binary exists
if [ ! -f "$BINARY_PATH" ]; then
    print "actionfile is not installed in $BIN_DIR" $YELLOW
    exit 0
fi

# Remove the binary
rm "$BINARY_PATH"

if [ $? -eq 0 ]; then
    print "actionfile has been successfully uninstalled!" $GREEN
else
    print "Failed to uninstall actionfile" $RED
    exit 1
fi

# Remind about PATH cleanup (optional)
print "\nNote: If you modified your PATH for actionfile, you may want to remove it from your shell configuration file (.zshrc, .bashrc, etc.)" $YELLOW
