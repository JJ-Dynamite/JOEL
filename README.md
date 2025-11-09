# 🧠 JOEL Programming Language

**JOEL** (Just-Objects-Events Language) is a polymodal programming language that can be **compiled** or **interpreted** based on a simple file header.

## 🚀 Quick Start

### Installation

**Option 1: Quick Install (Recommended)**

```bash
# Run the install script
chmod +x install.sh
./install.sh
```

**Option 2: Manual Install**

```bash
# Build the compiler
cargo build --release

# Install globally
sudo cp target/release/joel /usr/local/bin/joel
sudo chmod +x /usr/local/bin/joel
```

**Option 3: Local Install (No sudo)**

```bash
cargo build --release
export PATH="$PATH:$(pwd)/target/release"
```

See [INSTALL.md](INSTALL.md) for detailed instructions.

### Verify Installation

```bash
joel version
# Should output: JOEL Language v0.1.0
```

### Run a JOEL File

```bash
# Now you can use joel from anywhere!
joel run examples/hello.joel
```

### Example

Create a file `hello.joel`:

```joel
[Interpreted]

fn greet(name: str) -> str {
  return "Hello " + name
}

fn main() {
  print(greet("JOEL"))
  print("2 + 3 =", 2 + 3)
}

main()
```

Run it:

```bash
cargo run -- run hello.joel
```

## 📋 Features (Phase 1 - MVP)

✅ **Lexer** - Tokenizes JOEL source code  
✅ **Parser** - Parses tokens into an AST  
✅ **VM/Interpreter** - Executes `[Interpreted]` mode  
✅ **Header Detection** - Supports `[Compiled]` and `[Interpreted]` modes  
✅ **Target Hints** - Supports `[target native]`, `[target wasm32]`, `[target evm]`, etc.  
✅ **Basic Types** - Numbers, strings, booleans, lists, maps  
✅ **Control Flow** - if/else, while, for loops  
✅ **Functions** - Function definitions and calls  
✅ **Actors** - Actor-based concurrency (syntax support)  
✅ **Contracts** - Smart contract syntax (syntax support)  
✅ **Components** - UI component syntax (syntax support)  
✅ **Flows** - Workflow syntax (syntax support)  

## 🏗️ Project Structure

```
joel-lang/
├── src/
│   ├── main.rs      # CLI entry point
│   ├── lexer.rs     # Tokenizer
│   ├── parser.rs    # Parser
│   ├── ast.rs       # Abstract Syntax Tree
│   └── vm.rs        # Virtual Machine / Interpreter
├── examples/        # Example JOEL files
└── Cargo.toml       # Rust project config
```

## 📚 Language Syntax

### Header Modes

```joel
[Compiled]        # AOT/JIT compilation mode
[Interpreted]     # VM interpretation mode
[target wasm32]   # Optional target hint
```

### Basic Syntax

```joel
[Interpreted]

# Variables
let x = 10
let name: str = "JOEL"

# Functions
fn add(a: i32, b: i32) -> i32 {
  return a + b
}

# Control Flow
if x > 5 {
  print("High")
} else {
  print("Low")
}

# Loops
for i in range(0, 5) {
  print(i)
}

# Actors
actor Counter {
  state let n: i64 = 0
  
  fn inc() {
    self.n += 1
  }
}

# Contracts
[Compiled]
[target evm]

contract Vault {
  state let balance: uint256 = 0
  
  fn deposit() {
    balance += tx.value
  }
}
```

## 🛠️ Commands

```bash
# Run in interpreted mode
joel run <file.joel>

# Build for a target (not yet implemented)
joel build <file.joel> --target native
joel build <file.joel> --target wasm32
joel build <file.joel> --target evm

# Show version
joel version
```

## 🗺️ Roadmap

### Phase 1 (Current) ✅
- [x] Lexer
- [x] Parser
- [x] VM/Interpreter
- [x] Basic syntax support

### Phase 2 (Next)
- [ ] LLVM backend for `[Compiled]` mode
- [ ] WASM target compilation
- [ ] Ownership system
- [ ] Type checking

### Phase 3
- [ ] EVM bytecode generation
- [ ] Solana WASM target
- [ ] UI compiler (`joelui`)
- [ ] Container ops (`joelctl`)

### Phase 4
- [ ] Decentralized storage (`dstore`)
- [ ] AI/ML module (`ai`)
- [ ] Flow runtime (`flow`)
- [ ] Package manager (`joelpkg`)

## 📖 Documentation

Complete documentation is available:

- **Online**: [View Documentation](https://jj-dynamite.github.io/JOEL) (when deployed)
- **Local**: Build with `mdbook` (see [docs/README_DOCS.md](docs/README_DOCS.md))

### Quick Start with Documentation

```bash
# Install mdbook
cargo install mdbook

# Build documentation
cd docs
mdbook build

# Serve locally
mdbook serve
```

Then open http://localhost:3000 in your browser.

## 📖 Examples

See the `examples/` directory for:
- `hello.joel` - Basic syntax
- `actor.joel` - Actor example
- `contract.joel` - Smart contract example
- `flow.joel` - Workflow example

## 🤝 Contributing

This is an early-stage project. Contributions welcome!

## 📄 License

MIT License

---

**JOEL** - One Language, All Layers 🚀

