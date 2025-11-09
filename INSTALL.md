# 📦 Installing JOEL Language

## Quick Install

### Option 1: Using the Install Script (Recommended)

```bash
# Make script executable
chmod +x install.sh

# Run installation
./install.sh
```

This will:
1. Build the `joel` binary in release mode
2. Install it to `/usr/local/bin/joel`
3. Make it available globally

### Option 2: Manual Installation

```bash
# 1. Build the release binary
cargo build --release

# 2. Install to system PATH
sudo cp target/release/joel /usr/local/bin/joel
sudo chmod +x /usr/local/bin/joel
```

### Option 3: Local Installation (No sudo)

```bash
# 1. Build the release binary
cargo build --release

# 2. Add to your PATH (add to ~/.zshrc or ~/.bashrc)
export PATH="$PATH:$(pwd)/target/release"

# 3. Reload shell
source ~/.zshrc  # or source ~/.bashrc
```

## Verify Installation

```bash
# Check version
joel version

# Should output:
# JOEL Language v0.1.0
# A polymodal programming language
```

## Test It

```bash
# Run an example
joel run examples/hello.joel
```

## Uninstall

```bash
sudo rm /usr/local/bin/joel
```

## Troubleshooting

### "command not found: cargo"

Install Rust from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### "Permission denied"

Use `sudo` for system-wide installation, or install locally (Option 3).

### "joel: command not found"

Make sure `/usr/local/bin` is in your PATH:

```bash
echo $PATH | grep /usr/local/bin
```

If not, add it:

```bash
export PATH="$PATH:/usr/local/bin"
```

---

**That's it!** You now have `joel` installed as a system command. 🎉

