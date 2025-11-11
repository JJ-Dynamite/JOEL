# Documentation Plan

This document outlines the documentation strategy for JOEL, ensuring docs are updated as each phase is developed.

## Documentation Structure

```
docs/
├── pages/
│   ├── getting-started/     # Installation, quick start
│   ├── language/            # Language features
│   ├── guides/              # In-depth guides
│   ├── examples/            # Code examples
│   ├── stdlib/              # Standard library
│   ├── toolchain/           # CLI, build tools
│   ├── advanced/            # Advanced features
│   └── reference/           # Reference materials
```

## Phase-Based Documentation

### Phase 1: Core Language ✅

**Status**: Documented

- [x] Syntax Overview (`language/syntax-overview.mdx`)
- [x] Data Types (`language/data-types.mdx`)
- [x] Variables (`language/variables.mdx`)
- [x] Functions (`language/functions.mdx`)
- [x] Control Flow (`language/control-flow.mdx`)
- [x] Operators (`language/operators.mdx`)
- [x] Collections (`language/collections.mdx`)
- [x] Header Modes (`language/header-modes.mdx`)
- [x] CLI (`toolchain/cli.mdx`)
- [x] Examples (`examples/basic.mdx`)

### Phase 2: Compilation & Types ✅

**Status**: Documented

- [x] Type System Guide (`guides/type-system.mdx`)
- [x] Ownership System (`guides/ownership.mdx`)
- [x] Error Handling (`guides/error-handling.mdx`)
- [x] Updated Data Types page with Phase 2 features
- [x] Roadmap Status (`reference/roadmap-status.mdx`)

### Phase 3: Specialized Targets ✅

**Status**: Documented (basic)

- [x] CLI updated with Phase 3 features
- [x] Build examples created
- [ ] EVM Compilation Guide (detailed)
- [ ] Solana Target Guide (detailed)
- [ ] Native Compilation Guide (detailed)
- [ ] Cross-compilation Guide (detailed)
- [ ] Debugging Guide (update with debug symbols)

### Phase 4: Advanced Features

**Status**: Documented (toolchains)

- [x] UI Compiler Toolchain (`toolchain/joelui.mdx`)
- [x] Container Ops Toolchain (`toolchain/joelctl.mdx`)
- [x] Flow Runtime Toolchain (`toolchain/flow.mdx`)
- [ ] Actor System Guide (update `advanced/actors.mdx`)
- [ ] Async/Await Guide (`guides/async-await.mdx`)

### Phase 5: Ecosystem

**Status**: Documented (toolchain)

- [x] Package Manager Toolchain (`toolchain/joelpkg.mdx`)
- [ ] LSP Guide (`guides/lsp.mdx`)
- [ ] IDE Setup Guide (`guides/ide-setup.mdx`)
- [ ] Debugger Guide (update `guides/debugging.mdx`)
- [ ] Testing Guide (`guides/testing.mdx`)

### Phase 6: Database Programming

**Status**: Documented (planned features)

- [x] SQL Programming Guide (`guides/sql.mdx`)
- [x] SQL Toolchain Guide (`toolchain/joelsql.mdx`)
- [ ] Database Connectivity Guide
- [ ] SQL Performance Guide
- [ ] Examples (`examples/sql.mdx`)

### Phase 7: Quantum Programming

**Status**: Documented (toolchain)

- [x] Quantum Programming Toolchain (`toolchain/joelquantum.mdx`)
- [ ] Quantum Programming Guide (high-level)
- [ ] Quantum Backends Guide

### Phase 14: Quantum Programming (Qubit-Level) ✅

**Status**: Documented

- [x] Quantum Qubit Guide (`guides/quantum-qubit.mdx`)
- [x] Quantum Examples (`examples/quantum.mdx`)
- [x] Updated Data Types page
- [x] Updated Roadmap and Roadmap Status

### Phase 11: Distributed Systems

**Status**: Documented (toolchain)

- [x] Decentralized Storage Toolchain (`toolchain/dstore.mdx`)

### Phase 12: AI/ML Integration

**Status**: Documented (toolchain)

- [x] AI/ML Toolchain (`toolchain/ai.mdx`)

### Phase 8: Performance & Optimization

**Status**: Documented (toolchain)

- [x] Performance Profiling Toolchain (`toolchain/joelperf.mdx`)

### Phase 9: Security & Safety

**Status**: Documented (toolchain)

- [x] Security Analysis Toolchain (`toolchain/joelsec.mdx`)

### Phase 10: Interoperability

**Status**: Documented (toolchain)

- [x] Foreign Function Interface Toolchain (`toolchain/joelffi.mdx`)

### Phase 13: Real-time & Streaming

**Status**: Documented (toolchain)

- [x] Real-time & Streaming Toolchain (`toolchain/joelstream.mdx`)

### Phase 8-13: Other Future Phases

**Status**: To be documented as phases are developed

- [ ] Performance Optimization Guide (detailed)
- [ ] Security Guide (detailed)
- [ ] FFI Guide (detailed)
- [ ] Streaming Guide (detailed)

## Documentation Standards

### When to Update Docs

1. **During Development**: Update docs as features are implemented
2. **Before Release**: Ensure all new features are documented
3. **After Release**: Update based on user feedback

### Documentation Checklist

For each new feature:

- [ ] Create or update relevant guide page
- [ ] Add examples to examples section
- [ ] Update reference documentation
- [ ] Update roadmap status
- [ ] Add to FAQ if needed
- [ ] Update navigation (`_meta.js` files)
- [ ] Cross-reference related pages

### Code Examples

All code examples should:

- Be tested and working
- Include both `[Interpreted]` and `[Compiled]` examples when applicable
- Show error cases where relevant
- Be clear and concise
- Include comments explaining key concepts

## Maintenance

### Regular Updates

- **Weekly**: Update roadmap status
- **Per Phase**: Complete all phase documentation
- **Per Release**: Update version numbers and changelog

### Review Process

1. Technical review of accuracy
2. Editorial review for clarity
3. User testing for usability
4. Update based on feedback

## Contributing to Docs

See [CONTRIBUTING.md](https://github.com/JJ-Dynamite/JOEL) for guidelines on contributing documentation.

