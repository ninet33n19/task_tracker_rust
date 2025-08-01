#!/bin/bash
set -e

# Variables
BINARY_NAME="task-cli"
INSTALL_DIR="/usr/local/bin"
BIN_PATH="$INSTALL_DIR/$BINARY_NAME"

# Option 1: Build locally via cargo
echo "Building $BINARY_NAME..."
cargo build --release

echo "Installing $BINARY_NAME to $INSTALL_DIR"
sudo cp "target/release/$BINARY_NAME" "$BIN_PATH"
sudo chmod +x "$BIN_PATH"

echo "$BINARY_NAME installed successfully at $BIN_PATH"
echo "You can now run '$BINARY_NAME' from anywhere."
