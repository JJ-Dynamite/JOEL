# CLI Commands

The `joel` command-line tool provides various commands for working with JOEL.

## Command Overview

```bash
joel <command> [options]
```

## Commands

### run

Run a JOEL file in interpreted mode.

```bash
joel run <file.joel>
```

**Examples:**

```bash
# Run a simple program
joel run hello.joel

# Run with relative path
joel run examples/hello.joel

# Run with absolute path
joel run /path/to/program.joel
```

**Output:**

```
🚀 JOEL Runtime - Mode: Interpreted

[Program output]
```

### build

Build a JOEL file for a specific target.

```bash
joel build <file.joel> [--target <target>]
```

**Options:**

- `--target, -t` - Target platform (default: `native`)
  - `native` - Native binary
  - `wasm32` - WebAssembly
  - `evm` - Ethereum Virtual Machine
  - `wasm-solana` - Solana WASM

**Examples:**

```bash
# Build for native
joel build app.joel --target native

# Build for WebAssembly
joel build app.joel --target wasm32

# Build smart contract
joel build contract.joel --target evm
```

**Note:** Compilation backends are currently in development.

### version

Show version information.

```bash
joel version
```

**Output:**

```
JOEL Language v0.1.0
A polymodal programming language
```

## Command-Line Options

### Global Options

- `--help, -h` - Show help message
- `--version, -V` - Show version (same as `joel version`)

## Examples

### Basic Workflow

```bash
# 1. Create a file
echo '[Interpreted]
fn main() {
  print("Hello")
}
main()' > hello.joel

# 2. Run it
joel run hello.joel

# 3. Build it (when compilation is ready)
joel build hello.joel --target native
```

### Development Workflow

```bash
# Run in interpreted mode for fast iteration
joel run app.joel

# Build for production
joel build app.joel --target native

# Build for web
joel build app.joel --target wasm32
```

## Error Messages

### File Not Found

```
❌ Error reading file: No such file or directory
```

**Solution:** Check the file path and ensure the file exists.

### Missing Header

```
❌ Error: missing [Compiled] or [Interpreted] header
   Add [Interpreted] or [Compiled] at the top of your file
```

**Solution:** Add `[Interpreted]` or `[Compiled]` at the top of your JOEL file.

### Runtime Error

```
❌ Runtime error: [error message]
```

**Solution:** Check the error message and fix the issue in your code.

## Future Commands

Coming soon:

- `joel ui dev` - Start UI development server
- `joel chain build` - Build smart contracts
- `joel store put` - Upload to decentralized storage
- `joel ctl apply` - Apply Kubernetes deployments
- `joel pkg install` - Install packages

## Getting Help

```bash
# Show help
joel --help

# Show version
joel version

# Show command help
joel run --help
joel build --help
```

## Next Steps

- [Build System](build.md)
- [Package Management](packages.md)
- [Installation Guide](../getting-started/installation.md)

