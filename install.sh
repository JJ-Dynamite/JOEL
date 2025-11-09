#!/bin/bash

# JOEL Language Installation Script
# This script builds and installs the `joel` command globally

set -e

echo "🔨 Building JOEL Language..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Make sure Rust is installed: https://rustup.rs/"
    exit 1
fi

echo ""
echo "📦 Installing joel command..."

# Detect OS and install location
if [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS
    INSTALL_DIR="/usr/local/bin"
    SUDO_CMD="sudo"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux
    INSTALL_DIR="/usr/local/bin"
    SUDO_CMD="sudo"
else
    echo "⚠️  Unknown OS. Please install manually:"
    echo "   cp target/release/joel /usr/local/bin/joel"
    exit 1
fi

# Check if we need sudo
if [ -w "$INSTALL_DIR" ]; then
    SUDO_CMD=""
fi

# Copy binary
$SUDO_CMD cp target/release/joel "$INSTALL_DIR/joel"
$SUDO_CMD chmod +x "$INSTALL_DIR/joel"

echo "✅ Installation complete!"
echo ""
echo "Test it:"
echo "  joel version"
echo "  joel run examples/hello.joel"
echo ""

