# 🚀 JOEL Quick Start Guide

## Installation

### Prerequisites

- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)

```bash
# Verify Rust is installed
rustc --version
cargo --version
```

### Build JOEL

```bash
# Clone or navigate to the JOEL directory
cd /Users/Shared/JOEL

# Build the compiler
cargo build --release

# The binary will be at: target/release/joelc
```

### Add to PATH (Optional)

```bash
# Add to your shell profile (~/.zshrc, ~/.bashrc, etc.)
export PATH="$PATH:/Users/Shared/JOEL/target/release"

# Now you can use `joelc` from anywhere
joelc --help
```

## Your First JOEL Program

Create a file `hello.joel`:

```joel
[Interpreted]

fn main() {
  print("Hello, JOEL!")
  print("2 + 2 =", 2 + 2)
}

main()
```

Run it:

```bash
cargo run -- run hello.joel
# or
./target/release/joelc run hello.joel
```

Expected output:

```
🚀 JOEL Runtime - Mode: Interpreted

Hello, JOEL!
2 + 2 = 4
```

## Basic Syntax Examples

### Variables and Functions

```joel
[Interpreted]

let name = "JOEL"
let age = 24

fn greet(person: str) -> str {
  return "Hello, " + person
}

print(greet(name))
```

### Control Flow

```joel
[Interpreted]

let score = 85

if score >= 90 {
  print("Grade: A")
} elif score >= 80 {
  print("Grade: B")
} else {
  print("Grade: C")
}

# Loops
for i in range(0, 5) {
  print("Count:", i)
}
```

### Lists and Operations

```joel
[Interpreted]

let numbers = [1, 2, 3, 4, 5]
let sum = 0

for num in numbers {
  sum = sum + num
}

print("Sum:", sum)
```

## Running Examples

The `examples/` directory contains several example files:

```bash
# Basic hello world
cargo run -- run examples/hello.joel

# Arithmetic operations
cargo run -- run examples/arithmetic.joel

# Control flow
cargo run -- run examples/control_flow.joel

# Actor example (syntax only, execution coming soon)
cargo run -- run examples/actor.joel

# Contract example (syntax only, compilation coming soon)
cargo run -- run examples/contract.joel
```

## Command Reference

### Run (Interpreted Mode)

```bash
joelc run <file.joel>
```

Executes the file in interpreted mode using the JOEL VM.

### Build (Compiled Mode - Coming Soon)

```bash
joelc build <file.joel> --target native
joelc build <file.joel> --target wasm32
joelc build <file.joel> --target evm
```

Currently shows a placeholder message. Full compilation will be available in Phase 2.

## Language Modes

### Interpreted Mode

```joel
[Interpreted]

# Fast development, dynamic types
let x = 10
let y = "hello"
```

**Use when:**
- Prototyping
- AI/ML development
- Workflows
- Quick scripts

### Compiled Mode

```joel
[Compiled]

# Static types, high performance
let x: i32 = 10
let y: str = "hello"
```

**Use when:**
- Production systems
- Performance-critical code
- Smart contracts
- Native binaries

## Next Steps

1. **Read the README**: `README.md` for full documentation
2. **Check Architecture**: `ARCHITECTURE.md` for technical details
3. **Explore Examples**: `examples/` directory
4. **Experiment**: Create your own `.joel` files!

## Troubleshooting

### "command not found: cargo"

Install Rust from [rustup.rs](https://rustup.rs/)

### "Error reading file"

Make sure the file path is correct and the file exists.

### "Runtime error: ..."

Check your JOEL syntax. Common issues:
- Missing semicolons (optional but sometimes needed)
- Undefined variables
- Type mismatches

### "Undefined variable"

Make sure variables are declared before use:
```joel
let x = 10  # Declare first
print(x)    # Then use
```

## Contributing

JOEL is in early development. Contributions welcome!

See the roadmap in `README.md` for planned features.

---

**Happy coding with JOEL!** 🎉

