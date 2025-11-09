# Operators

JOEL supports various operators for different operations.

## Arithmetic Operators

```joel
let a = 10
let b = 3

a + b   # Addition: 13
a - b   # Subtraction: 7
a * b   # Multiplication: 30
a / b   # Division: 3 (integer) or 3.33... (float)
a % b   # Modulo: 1
```

### Type Behavior

```joel
let x: i32 = 10
let y: i32 = 3
let result: i32 = x / y  # Integer division: 3

let a: f64 = 10.0
let b: f64 = 3.0
let result: f64 = a / b  # Float division: 3.333...
```

## Comparison Operators

```joel
let a = 10
let b = 5

a == b   # Equal: false
a != b   # Not equal: true
a > b    # Greater than: true
a < b    # Less than: false
a >= b   # Greater or equal: true
a <= b   # Less or equal: false
```

## Logical Operators

```joel
let x = true
let y = false

x && y   # AND: false
x || y   # OR: true
!x       # NOT: false
```

### Short-Circuit Evaluation

```joel
# && stops if first is false
false && expensive_function()  # expensive_function not called

# || stops if first is true
true || expensive_function()   # expensive_function not called
```

## Assignment Operators

```joel
let x = 10

x += 5   # x = x + 5: 15 (coming soon)
x -= 3   # x = x - 3: 7 (coming soon)
x *= 2   # x = x * 2: 20 (coming soon)
x /= 2   # x = x / 2: 10 (coming soon)
x %= 3   # x = x % 3: 1 (coming soon)
```

**Note:** Currently use explicit assignment: `x = x + 5`

## String Operators

### Concatenation

```joel
let first = "Hello"
let second = "World"
let greeting = first + " " + second  # "Hello World"
```

### String Interpolation

```joel
# Coming soon
let name = "JOEL"
let greeting = "Hello, ${name}!"  # "Hello, JOEL!"
```

## Member Access

```joel
let person = {"name": "JOEL", "age": 24}
let name = person["name"]  # Access map value

# Coming soon
let name = person.name     # Dot notation
```

## Index Access

```joel
let list = [1, 2, 3, 4, 5]
let first = list[0]   # Access: 1
let last = list[4]    # Access: 5
```

## Function Call

```joel
fn add(a: i32, b: i32) -> i32 {
  return a + b
}

let result = add(5, 3)  # Call function: 8
```

## Operator Precedence

Operators are evaluated in this order (highest to lowest):

1. **Parentheses**: `()`
2. **Member/Index Access**: `.`, `[]`
3. **Unary**: `!`, `-`
4. **Multiplicative**: `*`, `/`, `%`
5. **Additive**: `+`, `-`
6. **Comparison**: `==`, `!=`, `<`, `>`, `<=`, `>=`
7. **Logical AND**: `&&`
8. **Logical OR**: `||`
9. **Assignment**: `=`, `+=`, `-=`, etc.

### Examples

```joel
let result = 2 + 3 * 4      # 14 (not 20)
let result = (2 + 3) * 4    # 20
let result = !true && false # false
let result = 5 > 3 && 2 < 4 # true
```

## Type-Specific Operators

### Numeric

```joel
let x: i32 = 10
let y: i32 = 3
x + y   # 13
x - y   # 7
x * y   # 30
x / y   # 3
x % y   # 1
```

### Boolean

```joel
let a = true
let b = false
a && b   # false
a || b   # true
!a       # false
```

### String

```joel
let s1 = "Hello"
let s2 = "World"
s1 + " " + s2  # "Hello World"
```

## Examples

### Calculator

```joel
let a = 15
let b = 3

print("a + b =", a + b)   # 18
print("a - b =", a - b)   # 12
print("a * b =", a * b)   # 45
print("a / b =", a / b)   # 5
print("a % b =", a % b)   # 0
```

### Comparisons

```joel
let x = 10
let y = 5

print("x > y:", x > y)     # true
print("x < y:", x < y)     # false
print("x == y:", x == y)   # false
print("x != y:", x != y)   # true
```

### Logical Operations

```joel
let age = 25
let has_license = true

let can_drive = age >= 18 && has_license  # true
let is_minor = age < 18 || !has_license   # false
```

## Next Steps

- [Control Flow](control-flow.md)
- [Functions](functions.md)
- [Data Types](data-types.md)

