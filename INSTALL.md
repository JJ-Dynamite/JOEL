# 📦 Installing JOEL Language

## Quick Install (Recommended)

Install JOEL with a single command:

```bash
curl -fsSL https://joel.val-x.com/install | bash
```

This will:
1. Check for Rust (install if needed)
2. Clone the JOEL repository
3. Build the `joel` binary from source
4. Install it to `/usr/local/bin/joel`
5. Make it available globally

## Manual Installation

If you prefer to build from source:

```bash
# Clone the repository
git clone https://github.com/JJ-Dynamite/JOEL.git
cd JOEL

# Build and install
cargo build --release
sudo cp target/release/joel /usr/local/bin/joel
sudo chmod +x /usr/local/bin/joel
```

## Local Installation (No sudo)

If you don't have sudo access:

```bash
# Clone and build
git clone https://github.com/JJ-Dynamite/JOEL.git
cd JOEL
cargo build --release

# Add to your PATH (add to ~/.zshrc or ~/.bashrc)
export PATH="$PATH:$(pwd)/target/release"
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

## Prerequisites

JOEL is written in Rust, so you'll need:

- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **Cargo** - Comes with Rust installation

The quick install script will automatically install Rust if it's not found.

## Troubleshooting

### "command not found: cargo"

Install Rust from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### "joel: command not found" after installation

Make sure `/usr/local/bin` is in your PATH:

```bash
echo $PATH | grep /usr/local/bin
```

If not, add it:

```bash
export PATH="$PATH:/usr/local/bin"
```

### Build errors

Make sure you have the latest Rust version:

```bash
rustup update
```

## Uninstall

To remove JOEL:

```bash
sudo rm /usr/local/bin/joel
```

## More Information

For detailed installation instructions, see the [Installation Guide](https://joel.val-x.com/getting-started/installation) in the documentation.

---

**That's it!** You now have `joel` installed as a system command. 🎉
