# Introduction to JOEL

**JOEL** (Just-Objects-Events Language) is a **polymodal programming language** that can be **compiled** or **interpreted** based on a simple file header.

## What is JOEL?

JOEL is designed to be a unified language that works across multiple domains:

- 🦀 **Systems Programming** - Rust-like performance with zero-cost abstractions
- 🐍 **AI/ML Development** - Python-like ergonomics for data science
- ⛓️ **Blockchain** - Solidity-like smart contract development
- 🎨 **UI Development** - React/Next.js-like component system
- 🔄 **Workflows** - n8n-like low-code automation
- 🐳 **DevOps** - Docker/K8s-like container orchestration
- ☁️ **Decentralized Storage** - IPFS/Arweave integration

## Key Features

### Polymodal Execution

JOEL can run in two modes, selected by a simple header:

```joel
[Interpreted]  # Fast development, dynamic types
```

or

```joel
[Compiled]     # High performance, static types
```

### Cross-Platform

Compile to multiple targets:

- **Native** - x86_64, ARM64 binaries
- **WASM** - Web and mobile applications
- **EVM** - Ethereum smart contracts
- **Solana** - Solana programs

### Modern Syntax

Clean, readable syntax inspired by Rust, Python, and JavaScript:

```joel
[Interpreted]

fn greet(name: str) -> str {
  return "Hello, " + name
}

fn main() {
  print(greet("JOEL"))
}
```

## Design Philosophy

> **One Language, All Layers**

JOEL aims to eliminate context switching between different languages and tools. Write system code, AI models, smart contracts, UIs, and workflows all in the same language.

## Who is JOEL For?

- **Full-Stack Developers** - One language for frontend, backend, and infrastructure
- **Blockchain Developers** - Smart contracts with familiar syntax
- **AI/ML Engineers** - Fast iteration with Python-like ergonomics
- **DevOps Engineers** - Infrastructure as code in JOEL
- **Startups** - Rapid prototyping across the entire stack

## Current Status

**Version 0.1.0** - Phase 1 MVP

✅ **Implemented:**
- Lexer and Parser
- VM/Interpreter for `[Interpreted]` mode
- Basic syntax (variables, functions, control flow)
- Built-in functions

🚧 **In Progress:**
- Compilation backends (LLVM, WASM, EVM)
- Type system
- Ownership system
- Standard library modules

## Next Steps

- [Install JOEL](getting-started/installation.md)
- [Write Your First Program](getting-started/first-program.md)
- [Explore the Syntax](language/syntax-overview.md)

---

**JOEL** - One Language, All Layers 🚀

