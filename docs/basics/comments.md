# Comments

Comments are notes in your code that are ignored by the compiler. They help explain what your code does.

## Single-Line Comments

Use `//` to create a comment that lasts until the end of the line:

```vyn
// This is a comment
let x: Int = 5  // This is also a comment
```

Everything after `//` on the same line is ignored by the compiler.

## Using Comments

### Explaining Code

```vyn
// Calculate the area of a rectangle
let width: Int = 10
let height: Int = 5
let area: Int = width * height
```

### Disabling Code

Comments can temporarily disable code:

```vyn
let x: Int = 10
// let y: Int = 20  // This line is disabled
let z: Int = 30
```

### Notes and TODOs

```vyn
// TODO: Add error handling
let @count: Int = 0
count = count + 1

// NOTE: This might overflow for large values
let big: Int = 1000 ^ 10
```

## Examples

```vyn
// Program: Calculate compound interest
// Author: Your Name

// Principal amount
let principal: Float = 1000.0

// Annual interest rate (5%)
let rate: Float = 0.05

// Number of years
let years: Int = 10

// Calculate final amount
// Formula: A = P * (1 + r)^t
let amount: Float = principal * ((1.0 + rate) ^ years)

stdout# amount  // Display result
```

```vyn
// Temperature converter
let celsius: Float = 25.0

// Convert to Fahrenheit: F = C * 9/5 + 32
let fahrenheit: Float = celsius * 9.0 / 5.0 + 32.0

stdout# fahrenheit  // Should output 77.0
```

## Best Practices

### Do

- Explain _why_ code does something, not _what_ it does
- Keep comments concise and relevant
- Update comments when code changes
- Use comments for complex logic

```vyn
// Use binary search because list is sorted
let index: Int = search(sortedList, target)
```

### Don't

- State the obvious
- Write overly long comments
- Leave outdated comments

```vyn
// Bad: States the obvious
let x: Int = 5  // Assign 5 to x

// Good: Explains purpose
let retryLimit: Int = 5  // Max attempts before giving up
```

## Next Steps

You've completed the Basics section! Continue to [Control Flow](../control-flow/index.md) to learn about if statements and program logic.
