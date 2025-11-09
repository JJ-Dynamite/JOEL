# 🧠 JOEL Programming Language

**JOEL** (Just-Objects-Events Language) is a **polymodal programming language** that can be **compiled** or **interpreted** based on a simple file header. Write once, run anywhere — from systems programming to AI, blockchain to UI.

## 🚀 Quick Start

### Installation

**Quick Install (Recommended)**

```bash
curl -fsSL https://joel.val-x.com/api/install | bash
```

**Manual Install**

```bash
# Clone the repository
git clone https://github.com/JJ-Dynamite/JOEL.git
cd JOEL

# Build and install
cargo build --release
sudo cp target/release/joel /usr/local/bin/joel
sudo chmod +x /usr/local/bin/joel
```

**Local Install (No sudo)**

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

### Run Your First Program

```bash
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
joel run hello.joel
```

## ✨ Features

### Phase 1 - MVP ✅

- ✅ **Lexer** - Tokenizes JOEL source code  
- ✅ **Parser** - Parses tokens into an AST  
- ✅ **VM/Interpreter** - Executes `[Interpreted]` mode  
- ✅ **Header Detection** - Supports `[Compiled]` and `[Interpreted]` modes  
- ✅ **Target Hints** - Supports `[target native]`, `[target wasm32]`, `[target evm]`, etc.  
- ✅ **Basic Types** - Numbers, strings, booleans, lists, maps  
- ✅ **Control Flow** - if/else, while, for loops  
- ✅ **Functions** - Function definitions and calls  
- ✅ **Actors** - Actor-based concurrency (syntax support)  
- ✅ **Contracts** - Smart contract syntax (syntax support)  
- ✅ **Components** - UI component syntax (syntax support)  
- ✅ **Flows** - Workflow syntax (syntax support)  

## 🏗️ Project Structure

```
JOEL/
├── src/              # Rust source code
│   ├── main.rs       # CLI entry point
│   ├── lexer.rs      # Tokenizer
│   ├── parser.rs     # Parser
│   ├── ast.rs        # Abstract Syntax Tree
│   └── vm.rs         # Virtual Machine / Interpreter
├── examples/         # Example JOEL files
├── docs/             # Nextra documentation site
│   ├── pages/        # Documentation pages (MDX)
│   ├── components/   # React components
│   └── styles/       # Custom styles
├── Cargo.toml        # Rust project config
└── Dockerfile        # Docker build for docs
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

# Build for a target (coming soon)
joel build <file.joel> --target native
joel build <file.joel> --target wasm32
joel build <file.joel> --target evm

# Show version
joel version
```

## 📖 Documentation

Complete documentation is available online:

🌐 **Live Documentation**: [https://joel.val-x.com](https://joel.val-x.com)

The documentation is built with **Nextra** (the same framework used for Next.js documentation) and features:

- ✨ Beautiful UI matching Next.js docs
- 🔍 Built-in search functionality
- 🌙 Dark mode support
- 📱 Mobile responsive
- ⚡ Fast page loads
- 📝 Markdown/MDX support
- 🎨 Syntax highlighting

### Local Development

To run the documentation locally:

```bash
# Navigate to docs directory
cd docs

# Install dependencies
npm install

# Start development server
npm run dev
```

Then open http://localhost:3000 in your browser.

## 📖 Examples

The `examples/` directory contains several example files:

- `hello.joel` - Basic syntax and functions
- `arithmetic.joel` - Math operations
- `control_flow.joel` - Control structures
- `actor.joel` - Actor-based concurrency
- `contract.joel` - Smart contract example
- `ui_component.joel` - UI component example
- `flow.joel` - Workflow example
- `deployment.joel` - Container deployment

Run any example:

```bash
joel run examples/hello.joel
```

See [TESTING.md](TESTING.md) for detailed test information.

## 🗺️ Roadmap

### Phase 1: Core Language (Current) ✅
- [x] Lexer - Tokenizes JOEL source code
- [x] Parser - Builds AST from tokens
- [x] VM/Interpreter - Executes `[Interpreted]` mode
- [x] Basic syntax support (variables, functions, control flow)
- [x] Built-in functions (`print`, `range`)
- [x] Documentation site with Nextra
- [x] CLI tool (`joel` command)
- [x] Example programs

### Phase 2: Compilation & Types (Next)
- [ ] LLVM backend for `[Compiled]` mode
- [ ] WASM target compilation
- [ ] Static type checking
- [ ] Type inference improvements
- [ ] Ownership system (Rust-like borrow checker)
- [ ] Error messages and diagnostics
- [ ] Standard library core modules

### Phase 3: Specialized Targets
- [ ] EVM bytecode generation for smart contracts
- [ ] Solana WASM target
- [ ] Native binary optimization
- [ ] Cross-compilation support
- [ ] Debug symbols and source maps

### Phase 4: Advanced Features
- [ ] UI compiler (`joelui`) - React/React Native output
- [ ] Container ops (`joelctl`) - Docker/K8s integration
- [ ] Flow runtime (`flow`) - Workflow execution
- [ ] Actor system implementation
- [ ] Async/await runtime
- [ ] Pattern matching improvements

### Phase 5: Ecosystem
- [ ] Package manager (`joelpkg`)
- [ ] Decentralized storage (`dstore`) - IPFS/Arweave
- [ ] AI/ML module (`ai`) - Tensor operations
- [ ] Language Server Protocol (LSP)
- [ ] IDE plugins (VSCode, etc.)
- [ ] Debugger and profiler
- [ ] Testing framework

## 📚 Additional Resources

- [Installation Guide](INSTALL.md) - Detailed installation instructions
- [Quick Start Guide](QUICKSTART.md) - Get started quickly
- [Architecture](ARCHITECTURE.md) - Technical architecture details
- [Testing Guide](TESTING.md) - Testing information

## 🤝 Contributing

This is an early-stage project. Contributions welcome!

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

MIT License

---

**JOEL** - One Language, All Layers 🚀
