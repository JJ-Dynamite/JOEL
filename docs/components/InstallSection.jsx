'use client'

import { useState } from 'react'
import CopyButton from './CopyButton'

export default function InstallSection() {
  const [activeTab, setActiveTab] = useState('linux-macos')

  const linuxMacosCommand = 'curl -fsSL https://joel.val-x.com/api/install | bash'
  const windowsInstructions = `# Download JOEL for Windows
# Visit: https://github.com/JJ-Dynamite/JOEL/releases/latest
# Download joel-windows-x64.exe (or appropriate version)

# Option 1: Direct download and install
# 1. Download the latest release from GitHub
# 2. Rename to joel.exe
# 3. Move to a directory in your PATH (e.g., C:\\Users\\YourName\\bin)
# 4. Add that directory to your PATH in System Environment Variables

# Option 2: Using PowerShell
# Download and add to PATH:
$url = "https://github.com/JJ-Dynamite/JOEL/releases/latest/download/joel-windows-x64.exe"
$output = "$env:USERPROFILE\\bin\\joel.exe"
New-Item -ItemType Directory -Force -Path "$env:USERPROFILE\\bin"
Invoke-WebRequest -Uri $url -OutFile $output
$env:Path += ";$env:USERPROFILE\\bin"
[Environment]::SetEnvironmentVariable("Path", $env:Path, [EnvironmentVariableTarget]::User)

# Verify installation:
joel version`

  const installScript = `#!/bin/bash

# JOEL Language Remote Installation Script
# This script can be piped from curl: curl -fsSL https://joel.val-x.com/api/install | bash

set -e

# Colors for output
RED='\\033[0;31m'
GREEN='\\033[0;32m'
YELLOW='\\033[1;33m'
BLUE='\\033[0;34m'
NC='\\033[0m' # No Color

echo -e "\${BLUE}🧠 JOEL Language Installer\${NC}"
echo ""

# Check if git is available
if ! command -v git &> /dev/null; then
    echo -e "\${RED}❌ Git is not installed. Please install Git first.\${NC}"
    exit 1
fi

# Create temporary directory
TEMP_DIR=\$(mktemp -d)
trap "rm -rf \$TEMP_DIR" EXIT

echo -e "\${BLUE}📦 Downloading JOEL Language...\${NC}"

# Detect architecture
ARCH=\$(uname -m)
OS=\$(uname -s | tr '[:upper:]' '[:lower:]')

if [ "\$OS" = "darwin" ]; then
    OS="macos"
fi

# Download pre-built binary
BINARY_URL="https://github.com/JJ-Dynamite/JOEL/releases/latest/download/joel-\${OS}-\${ARCH}"

if curl -fsSL "\$BINARY_URL" -o "\$TEMP_DIR/joel" 2>/dev/null; then
    echo -e "\${GREEN}✅ Downloaded JOEL\${NC}"
    chmod +x "\$TEMP_DIR/joel"
    BINARY_PATH="\$TEMP_DIR/joel"
else
    echo -e "\${YELLOW}⚠️  Pre-built binary not available.\${NC}"
    echo -e "\${YELLOW}   Please visit https://joel.val-x.com/getting-started/installation for manual installation.\${NC}"
    exit 1
fi

echo ""
echo -e "\${BLUE}📦 Installing joel command...\${NC}"

# Detect OS and install location
if [[ "\$OSTYPE" == "darwin"* ]]; then
    INSTALL_DIR="/usr/local/bin"
elif [[ "\$OSTYPE" == "linux-gnu"* ]]; then
    INSTALL_DIR="/usr/local/bin"
else
    echo -e "\${YELLOW}⚠️  Unknown OS. Installing to ~/.local/bin\${NC}"
    INSTALL_DIR="\$HOME/.local/bin"
    mkdir -p "\$INSTALL_DIR"
fi

# Check if we need sudo
if [ -w "\$INSTALL_DIR" ]; then
    cp "\$BINARY_PATH" "\$INSTALL_DIR/joel"
    chmod +x "\$INSTALL_DIR/joel"
    SUDO_USED=false
else
    sudo cp "\$BINARY_PATH" "\$INSTALL_DIR/joel"
    sudo chmod +x "\$INSTALL_DIR/joel"
    SUDO_USED=true
fi

# Verify installation
if command -v joel &> /dev/null; then
    echo ""
    echo -e "\${GREEN}✅ Installation complete!\${NC}"
    echo ""
    echo "Test it:"
    echo "  joel version"
    echo ""
    
    # Show version
    joel version
else
    echo -e "\${YELLOW}⚠️  Installation complete, but joel command not found in PATH.\${NC}"
    if [ "\$SUDO_USED" = false ] && [ "\$INSTALL_DIR" != "/usr/local/bin" ]; then
        echo "Add to your PATH:"
        echo "  export PATH=\\"\\\$PATH:\$INSTALL_DIR\\""
        echo "  # Add to ~/.bashrc or ~/.zshrc for persistence"
    fi
fi

echo ""
echo -e "\${GREEN}🚀 JOEL is ready to use!\${NC}"`

  const renderContent = () => {
    switch (activeTab) {
      case 'linux-macos':
        return (
          <div className="install-command">
            <code>$ {linuxMacosCommand}</code>
            <CopyButton text={linuxMacosCommand} />
          </div>
        )
      case 'windows':
        return (
          <div className="install-command">
            <pre className="install-script-content">{windowsInstructions}</pre>
            <CopyButton text={windowsInstructions} />
          </div>
        )
      case 'script':
        return (
          <div className="install-command">
            <pre className="install-script-content">{installScript}</pre>
            <CopyButton text={installScript} />
          </div>
        )
      default:
        return null
    }
  }

  return (
    <div className="install-section">
      <div className="install-title">Install JOEL</div>
      <div className="platform-tabs">
        <button
          className={`platform-tab ${activeTab === 'linux-macos' ? 'active' : ''}`}
          onClick={() => setActiveTab('linux-macos')}
        >
          Linux & macOS
        </button>
        <button
          className={`platform-tab ${activeTab === 'windows' ? 'active' : ''}`}
          onClick={() => setActiveTab('windows')}
        >
          Windows
        </button>
        <button
          className={`platform-tab ${activeTab === 'script' ? 'active' : ''}`}
          onClick={() => setActiveTab('script')}
        >
          View install script
        </button>
      </div>
      {renderContent()}
    </div>
  )
}

