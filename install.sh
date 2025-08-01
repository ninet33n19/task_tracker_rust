#!/bin/bash

# Build the release binary
cargo build --release

# Path to the built binary
BINARY_PATH="./target/release/task-cli"

# Check if the binary was built successfully
if [ ! -f "$BINARY_PATH" ]; then
    echo "Build failed: $BINARY_PATH not found."
    exit 1
fi

# Optionally copy the binary to /usr/local/bin (requires sudo)
echo "Copying $BINARY_PATH to /usr/local/bin/"
sudo cp "$BINARY_PATH" /usr/local/bin/

echo "Release binary installed to /usr/local/bin/"
