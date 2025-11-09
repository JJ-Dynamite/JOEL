# Syntax Overview

This page provides a comprehensive overview of JOEL syntax.

## File Structure

Every JOEL file starts with an execution mode header:

```joel
[Interpreted]  # or [Compiled]
```

Optional target hint:

```joel
[target wasm32]  # native, wasm32, evm, wasm-solana
```

## Comments

```joel
# Single-line comment

// Also single-line comment

/*
  Multi-line comment
  (coming soon)
*/
```

## Variables

```joel
let name = "JOEL"           # Type inferred
let age: i32 = 24           # Explicit type
let active: bool = true     # Boolean
```

## Constants

```joel
const PI: f64 = 3.14159
const MAX_SIZE: i32 = 100
```

## Functions

```joel
# Simple function
fn greet() {
  print("Hello")
}

# Function with parameters
fn add(a: i32, b: i32) -> i32 {
  return a + b
}

# Function with return type
fn get_name() -> str {
  return "JOEL"
}
```

## Control Flow

### If Statements

```joel
if condition {
  # code
}

if condition {
  # code
} else {
  # code
}

if condition {
  # code
} elif other_condition {
  # code
} else {
  # code
}
```

### While Loops

```joel
while condition {
  # code
}
```

### For Loops

```joel
# Range-based
for i in range(0, 10) {
  print(i)
}

# List iteration
let items = [1, 2, 3]
for item in items {
  print(item)
}
```

## Operators

### Arithmetic

```joel
let a = 10
let b = 5

a + b   # Addition: 15
a - b   # Subtraction: 5
a * b   # Multiplication: 50
a / b   # Division: 2
a % b   # Modulo: 0
```

### Comparison

```joel
a == b   # Equal: false
a != b   # Not equal: true
a > b    # Greater than: true
a < b    # Less than: false
a >= b   # Greater or equal: true
a <= b   # Less or equal: false
```

### Logical

```joel
true && false   # AND: false
true || false   # OR: true
!true           # NOT: false
```

### String Concatenation

```joel
let greeting = "Hello " + "World"  # "Hello World"
```

## Data Types

### Primitives

```joel
let num: i32 = 42        # 32-bit integer
let big: i64 = 1000000   # 64-bit integer
let float: f64 = 3.14    # 64-bit float
let text: str = "JOEL"   # String
let flag: bool = true   # Boolean
```

### Collections

```joel
# Lists
let numbers = [1, 2, 3, 4, 5]
let names = ["Alice", "Bob", "Charlie"]

# Maps (dictionaries)
let person = {
  "name": "JOEL",
  "age": 24
}
```

## Pattern Matching

```joel
match value {
  "red" => print("Stop"),
  "green" => print("Go"),
  "yellow" => print("Slow"),
  _ => print("Unknown"),
}
```

## Modules

```joel
module my_module

# Import other modules
import std
import ai as ml
```

## Error Handling

```joel
# Result type (coming soon)
fn read_file(path: str) -> Result<Bytes, Error> {
  let f = File.open(path)?
  defer f.close()
  return f.read_all()
}
```

## Next Steps

- [Data Types](data-types.md) - Detailed type information
- [Functions](functions.md) - Function reference
- [Control Flow](control-flow.md) - Control structures
- [Operators](operators.md) - Complete operator reference

