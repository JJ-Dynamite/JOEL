# Data Types

JOEL supports various data types for different use cases.

## Primitive Types

### Integers

```joel
let small: i32 = 42        # 32-bit signed integer
let large: i64 = 1000000  # 64-bit signed integer
let unsigned: u32 = 100   # 32-bit unsigned integer
let big: u64 = 999999     # 64-bit unsigned integer
```

### Floating Point

```joel
let single: f32 = 3.14    # 32-bit float
let double: f64 = 3.14159 # 64-bit float (default)
```

### Boolean

```joel
let active: bool = true
let disabled: bool = false
```

### String

```joel
let name: str = "JOEL"
let greeting = "Hello, World"
let multiline = "Line 1
Line 2"
```

## Collection Types

### Lists

```joel
let numbers: list[i32] = [1, 2, 3, 4, 5]
let names: list[str] = ["Alice", "Bob"]
let mixed = [1, "hello", true]  # Type inferred
```

**List Operations:**

```joel
let items = [1, 2, 3]
let first = items[0]      # Access: 1
let len = items.length()  # Length: 3
```

### Maps (Dictionaries)

```joel
let person: map[str, str] = {
  "name": "JOEL",
  "age": "24"
}

let scores: map[str, i32] = {
  "Alice": 95,
  "Bob": 87
}
```

**Map Operations:**

```joel
let person = {"name": "JOEL", "age": 24}
let name = person["name"]  # Access: "JOEL"
person["city"] = "NYC"     # Add/Update
```

## Special Types

### Option

```joel
let value: Option[i32] = Some(10)
let none: Option[i32] = None

# Pattern matching (coming soon)
match value {
  Some(x) => print(x),
  None => print("No value"),
}
```

### Result

```joel
fn read_file(path: str) -> Result<Bytes, Error> {
  # Returns Ok(value) or Err(error)
}
```

### Bytes

```joel
let data: Bytes = bytes("Hello")
let raw = bytes([0x48, 0x65, 0x6c, 0x6c, 0x6f])
```

## Type Inference

JOEL can infer types automatically:

```joel
let x = 10           # Inferred as i32
let y = 3.14         # Inferred as f64
let z = "hello"      # Inferred as str
let flag = true      # Inferred as bool
```

## Type Annotations

Explicit type annotations:

```joel
let x: i32 = 10
let y: f64 = 3.14
let name: str = "JOEL"
```

## Type Casting

```joel
let x: i32 = 10
let y: f64 = x as f64  # Cast to float

let num: f64 = 3.14
let int: i32 = num as i32  # Cast to integer
```

## Type Checking

### Compiled Mode

In `[Compiled]` mode, types are checked at compile time:

```joel
[Compiled]

let x: i32 = 10
let y: str = "hello"
let z = x + y  # Error: type mismatch
```

### Interpreted Mode

In `[Interpreted]` mode, types are checked at runtime:

```joel
[Interpreted]

let x = 10
let y = "hello"
let z = x + y  # Runtime error
```

## Next Steps

- [Variables](variables.md)
- [Functions](functions.md)
- [Operators](operators.md)

