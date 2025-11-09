# 🧪 Testing JOEL Examples

## Quick Test Guide

### Prerequisites

1. **Build the joel binary:**
   ```bash
   cargo build --release
   ```

2. **Or install globally:**
   ```bash
   ./install.sh
   ```

### Running Tests

**Option 1: Use the test script**
```bash
./test_examples.sh
```

**Option 2: Test individual files**
```bash
# If binary is built locally
./target/release/joel run examples/hello.joel

# If installed globally
joel run examples/hello.joel
```

---

## Example Files Status

### ✅ Fully Working Examples

These examples should work completely:

1. **hello.joel** - Basic syntax demo
   ```bash
   joel run examples/hello.joel
   ```
   - ✅ Variables, functions, conditionals, loops
   - ✅ String operations
   - ✅ Built-in functions

2. **arithmetic.joel** - Math operations
   ```bash
   joel run examples/arithmetic.joel
   ```
   - ✅ All arithmetic operations
   - ✅ Comparison operators
   - ⚠️ Note: Type casting (`as f64`) is parsed but may not work fully

3. **test_basic.joel** - Minimal test
   ```bash
   joel run examples/test_basic.joel
   ```
   - ✅ All basic features verified

4. **simple_test.joel** - Comprehensive test
   ```bash
   joel run examples/simple_test.joel
   ```
   - ✅ Full feature coverage

### ⚠️ Partially Working Examples

These examples have syntax that's parsed but not fully executed:

5. **control_flow.joel** - Control structures
   ```bash
   joel run examples/control_flow.joel
   ```
   - ✅ If/elif/else works
   - ✅ For loops work
   - ⚠️ While loop with `count += 1` - assignment operators not yet implemented
   - ✅ List iteration works

### 📝 Syntax Only Examples

These examples parse correctly but execution/compilation not yet implemented:

6. **actor.joel** - Actor syntax
   - ✅ Parses correctly
   - ❌ Actor execution not yet implemented

7. **contract.joel** - Smart contract
   - ✅ Parses correctly
   - ❌ EVM compilation not yet implemented

8. **ui_component.joel** - UI component
   - ✅ Parses correctly
   - ❌ UI rendering not yet implemented

9. **flow.joel** - Workflow
   - ✅ Parses correctly
   - ❌ Flow execution not yet implemented

10. **deployment.joel** - Container deployment
    - ✅ Parses correctly
    - ❌ Deployment not yet implemented

---

## Expected Outputs

### hello.joel Output

```
🚀 JOEL Runtime - Mode: Interpreted

📦 Module: hello
Hello JOEL
2 + 3 = 5
x + y = 30
x is greater than 5
i = 0
i = 1
i = 2
i = 3
i = 4
```

### arithmetic.joel Output

```
🚀 JOEL Runtime - Mode: Interpreted

📦 Module: math
=== Arithmetic Operations ===
x = 15
y = 3

x + y = 18
x - y = 12
x * y = 45
x / y = 5

x > y: true
x < y: false
x == y: false
x != y: true
```

### test_basic.joel Output

```
🚀 JOEL Runtime - Mode: Interpreted

=== Basic Test ===
x = 10
y = 20
x + y = 30
Hello JOEL
x is less than y
Counting:
  0
  1
  2
List: [1, 2, 3]
✅ Basic test complete!
```

---

## Known Issues

1. **Assignment Operators**: `+=`, `-=`, `*=`, `/=` not yet implemented
   - Workaround: Use `count = count + 1` instead of `count += 1`

2. **Type Casting**: `as f64` syntax parsed but not fully implemented
   - May cause runtime errors in some cases

3. **While Loops with Assignment**: Will fail if using `+=` operator
   - Example: `control_flow.joel` line 24

---

## Test Checklist

- [x] Basic variables
- [x] String operations
- [x] Arithmetic operations
- [x] Comparison operators
- [x] If/elif/else statements
- [x] For loops with range
- [x] List literals and iteration
- [x] Function definitions and calls
- [x] Module declarations
- [ ] Assignment operators (`+=`, etc.)
- [ ] Type casting (`as Type`)
- [ ] Actor execution
- [ ] Contract compilation
- [ ] UI rendering
- [ ] Flow execution

---

## Contributing Tests

To add a new test:

1. Create a `.joel` file in `examples/`
2. Add `[Interpreted]` header
3. Write test code
4. Document expected output in this file
5. Run and verify: `joel run examples/your_test.joel`

---

**Happy Testing!** 🎉

