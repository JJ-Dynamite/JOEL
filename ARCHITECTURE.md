# 🏗️ JOEL Language Architecture

## Overview

JOEL is a polymodal programming language built in Rust. The architecture follows a traditional compiler/interpreter pipeline:

```
Source Code → Lexer → Parser → AST → VM/Compiler → Execution
```

## Components

### 1. Lexer (`src/lexer.rs`)

**Purpose**: Tokenizes source code into a stream of tokens.

**Key Features**:
- Header detection (`[Compiled]`, `[Interpreted]`, `[target ...]`)
- Support for comments (`//` and `#`)
- String literals with escape sequences
- Number parsing (integers and floats)
- Keyword recognition
- Operator tokenization

**Token Types**:
- Keywords: `let`, `fn`, `if`, `else`, `while`, `for`, etc.
- Literals: Numbers, strings, booleans
- Operators: `+`, `-`, `*`, `/`, `==`, `!=`, etc.
- Delimiters: `{`, `}`, `(`, `)`, `[`, `]`, etc.

### 2. Parser (`src/parser.rs`)

**Purpose**: Builds an Abstract Syntax Tree (AST) from tokens.

**Parsing Strategy**: Recursive descent parser with precedence handling.

**Grammar Highlights**:
- Expressions: Binary ops, unary ops, function calls, member access
- Statements: Variable declarations, functions, control flow, blocks
- Declarations: Modules, imports, actors, contracts, components, flows

**Precedence** (lowest to highest):
1. Assignment
2. Or (`||`)
3. And (`&&`)
4. Equality (`==`, `!=`)
5. Comparison (`<`, `>`, `<=`, `>=`)
6. Term (`+`, `-`)
7. Factor (`*`, `/`, `%`)
8. Unary (`!`, `-`)
9. Primary (literals, identifiers, calls)

### 3. AST (`src/ast.rs`)

**Purpose**: Represents the program structure.

**Key Types**:
- `Expr`: Expressions (numbers, strings, binary ops, calls, etc.)
- `Stmt`: Statements (let, if, while, functions, etc.)
- `Program`: Top-level program with mode and target info
- `ExecutionMode`: `Compiled`, `Interpreted`, or `Unknown`

### 4. VM (`src/vm.rs`)

**Purpose**: Executes programs in `[Interpreted]` mode.

**Features**:
- Stack-based execution
- Variable scoping (global + local frames)
- Built-in functions (`print`, `range`)
- Type system (dynamic in interpreted mode)
- Error handling

**Value Types**:
- `Number` (f64)
- `String`
- `Boolean`
- `List` (Vec<Value>)
- `Map` (HashMap<String, Value>)
- `Function`
- `None`

**Execution Flow**:
1. Initialize VM with built-ins
2. Execute statements sequentially
3. Evaluate expressions recursively
4. Handle control flow (if/while/for)
5. Manage variable scopes with stack frames

### 5. CLI (`src/main.rs`)

**Purpose**: Command-line interface for the compiler.

**Commands**:
- `joelc run <file.joel>` - Run in interpreted mode
- `joelc build <file.joel> --target <target>` - Build for target (future)

## Data Flow

```
┌─────────────┐
│ Source File │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Lexer     │ → Tokens
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Parser    │ → AST
└──────┬──────┘
       │
       ├─────────────────┐
       │                 │
       ▼                 ▼
┌─────────────┐    ┌─────────────┐
│     VM      │    │  Compiler   │ (Future)
│(Interpreted)│    │  (Compiled) │
└──────┬──────┘    └─────────────┘
       │
       ▼
┌─────────────┐
│  Execution  │
└─────────────┘
```

## Current Limitations

1. **No type checking** - Types are checked at runtime only
2. **No ownership system** - Rust-like ownership not yet implemented
3. **Limited built-ins** - Only `print` and `range` implemented
4. **No compilation** - `[Compiled]` mode not yet implemented
5. **No FFI** - Foreign function interface not yet implemented
6. **Actor execution** - Syntax parsed but not executed
7. **Contract compilation** - Syntax parsed but not compiled to EVM

## Future Architecture

### Phase 2: Compilation

- **LLVM Backend**: Generate LLVM IR for native targets
- **WASM Backend**: Generate WebAssembly for web/mobile
- **Type System**: Static type checking for compiled mode
- **Ownership**: Borrow checker for memory safety

### Phase 3: Specialized Backends

- **EVM Backend**: Compile contracts to EVM bytecode
- **Solana Backend**: Compile to Solana WASM
- **UI Compiler**: Transform components to React/React Native
- **Flow Runtime**: Execute workflow graphs

### Phase 4: Tooling

- **Package Manager**: `joelpkg` for dependency management
- **LSP Server**: Language server for IDE support
- **Debugger**: Step-through debugging
- **Profiler**: Performance analysis

## Design Decisions

1. **Rust Implementation**: Chosen for performance, safety, and ecosystem
2. **Recursive Descent Parser**: Simple, maintainable, easy to extend
3. **Stack-based VM**: Traditional approach, easy to implement
4. **Dynamic Types (Interpreted)**: Fast development, Python-like ergonomics
5. **Static Types (Compiled)**: Performance, Rust-like safety (future)

## Testing Strategy

- Unit tests for lexer, parser, VM
- Integration tests with example files
- Property-based tests for edge cases
- Fuzzing for parser robustness

## Performance Considerations

- **Lexer**: Single pass, O(n) time complexity
- **Parser**: Recursive descent, O(n) for well-formed code
- **VM**: Stack-based, efficient for interpreted execution
- **Future**: JIT compilation for hot paths in interpreted mode

