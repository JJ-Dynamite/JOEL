# Control Flow

JOEL provides various control flow constructs for program logic.

## If Statements

### Basic If

```joel
if condition {
  print("Condition is true")
}
```

### If-Else

```joel
if x > 10 {
  print("x is greater than 10")
} else {
  print("x is not greater than 10")
}
```

### If-Elif-Else

```joel
if score >= 90 {
  print("Grade: A")
} elif score >= 80 {
  print("Grade: B")
} elif score >= 70 {
  print("Grade: C")
} else {
  print("Grade: F")
}
```

## While Loops

```joel
let count = 0
while count < 5 {
  print("Count:", count)
  count = count + 1
}
```

### Infinite Loop

```joel
while true {
  # Do something
  if should_break {
    break  # Coming soon
  }
}
```

## For Loops

### Range-Based Loop

```joel
for i in range(0, 5) {
  print("i =", i)
}

# Output:
# i = 0
# i = 1
# i = 2
# i = 3
# i = 4
```

### List Iteration

```joel
let numbers = [10, 20, 30]
for num in numbers {
  print("Number:", num)
}

# Output:
# Number: 10
# Number: 20
# Number: 30
```

### Map Iteration

```joel
let person = {"name": "JOEL", "age": 24}
for key, value in person {
  print(key, "=", value)
}
```

## Pattern Matching

```joel
match value {
  "red" => print("Stop"),
  "green" => print("Go"),
  "yellow" => print("Slow"),
  _ => print("Unknown color"),
}
```

### Matching Numbers

```joel
match x {
  0 => print("Zero"),
  1 => print("One"),
  2 => print("Two"),
  _ => print("Other"),
}
```

### Matching with Guards

```joel
# Coming soon
match x {
  n if n > 10 => print("Large"),
  n if n > 5 => print("Medium"),
  _ => print("Small"),
}
```

## Break and Continue

```joel
# Coming soon
for i in range(0, 10) {
  if i == 5 {
    break  # Exit loop
  }
  if i % 2 == 0 {
    continue  # Skip to next iteration
  }
  print(i)
}
```

## Nested Control Flow

```joel
for i in range(0, 3) {
  if i % 2 == 0 {
    print("Even:", i)
  } else {
    print("Odd:", i)
  }
}
```

## Examples

### Grade Calculator

```joel
fn calculate_grade(score: i32) -> str {
  if score >= 90 {
    return "A"
  } elif score >= 80 {
    return "B"
  } elif score >= 70 {
    return "C"
  } elif score >= 60 {
    return "D"
  } else {
    return "F"
  }
}
```

### Number Guessing

```joel
let target = 42
let guess = 0

while guess != target {
  # Get guess from user (coming soon)
  if guess < target {
    print("Too low")
  } elif guess > target {
    print("Too high")
  }
}
print("Correct!")
```

### List Processing

```joel
let numbers = [1, 2, 3, 4, 5]
let sum = 0

for num in numbers {
  sum = sum + num
}

print("Sum:", sum)
```

## Best Practices

1. **Use early returns**: Reduce nesting
2. **Prefer for loops**: When iterating over collections
3. **Use pattern matching**: For multiple conditions
4. **Avoid deep nesting**: Keep code readable

## Next Steps

- [Operators](operators.md)
- [Functions](functions.md)
- [Collections](collections.md)

