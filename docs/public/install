#!/bin/bash

# JOEL Language Remote Installation Script
# This script can be piped from curl: curl -fsSL https://joel.val-x.com/install | bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧠 JOEL Language Installer${NC}"
echo ""

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${YELLOW}⚠️  Rust is not installed.${NC}"
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✅ Rust installed${NC}"
    echo ""
fi

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo not found. Please install Rust: https://rustup.rs/${NC}"
    exit 1
fi

# Create temporary directory
TEMP_DIR=$(mktemp -d)
trap "rm -rf $TEMP_DIR" EXIT

echo -e "${BLUE}📦 Cloning JOEL repository...${NC}"
cd "$TEMP_DIR"
git clone --depth 1 https://github.com/JJ-Dynamite/JOEL.git joel-lang
cd joel-lang

echo ""
echo -e "${BLUE}🔨 Building JOEL Language...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Build failed.${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}📦 Installing joel command...${NC}"

# Detect OS and install location
if [[ "$OSTYPE" == "darwin"* ]]; then
    INSTALL_DIR="/usr/local/bin"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    INSTALL_DIR="/usr/local/bin"
else
    echo -e "${YELLOW}⚠️  Unknown OS. Installing to ~/.local/bin${NC}"
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Check if we need sudo
if [ -w "$INSTALL_DIR" ]; then
    cp target/release/joel "$INSTALL_DIR/joel"
    chmod +x "$INSTALL_DIR/joel"
    SUDO_USED=false
else
    sudo cp target/release/joel "$INSTALL_DIR/joel"
    sudo chmod +x "$INSTALL_DIR/joel"
    SUDO_USED=true
fi

# Verify installation
if command -v joel &> /dev/null; then
    echo ""
    echo -e "${GREEN}✅ Installation complete!${NC}"
    echo ""
    echo "Test it:"
    echo "  joel version"
    echo ""
    
    # Show version
    joel version
else
    echo -e "${YELLOW}⚠️  Installation complete, but joel command not found in PATH.${NC}"
    if [ "$SUDO_USED" = false ] && [ "$INSTALL_DIR" != "/usr/local/bin" ]; then
        echo "Add to your PATH:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
        echo "  # Add to ~/.bashrc or ~/.zshrc for persistence"
    fi
fi

echo ""
echo -e "${GREEN}🚀 JOEL is ready to use!${NC}"

