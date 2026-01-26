# Expressions

Expressions are combinations of values, variables, and operators that evaluate to a single value. Every expression in Vyn has a type.

## Literals

The simplest expressions are literal values:

```vyn
42              // Int literal
3.14            // Float literal
true            // Bool literal
"Hello"         // String literal
[1, 2, 3]       // Array literal
```

## Variable Expressions

Variable names are expressions that evaluate to their value:

```vyn
let x: Int = 10
let y: Int = x  // x is an expression
```

## Array Expressions

Array literals create fixed-size arrays:

```vyn
let numbers: [3]Int = [1, 2, 3]
let prices: [2]Float = [9.99, 14.99]
let flags: [4]Bool = [true, false, true, true]
let names: [2]String = ["Alice", "Bob"]
```

Array expressions must match their declared size exactly:

```vyn
let valid: [3]Int = [10, 20, 30]     // OK
let invalid: [3]Int = [10, 20]       // Error! Expected 3 elements
```

## Arithmetic Expressions

Combine values using arithmetic operators:

```vyn
let sum: Int = 5 + 3           // 8
let difference: Int = 10 - 4   // 6
let product: Int = 6 * 7       // 42
let quotient: Float = 15.0 / 3.0  // 5.0
let power: Int = 2 ^ 8         // 256
```

## Comparison Expressions

Compare values to get a boolean result:

```vyn
let is_equal: Bool = 5 == 5        // true
let not_equal: Bool = 3 != 7       // true
let greater: Bool = 10 > 5        // true
let less: Bool = 2 < 8            // true
let greater_equal: Bool = 5 >= 5   // true
let less_equal: Bool = 3 <= 2      // false
```

## Parentheses

Use parentheses to control evaluation order:

```vyn
let result1: Int = 2 + 3 * 4      // 14 (multiplication first)
let result2: Int = (2 + 3) * 4    // 20 (parentheses first)
```

## Complex Expressions

Combine multiple operations:

```vyn
let x: Int = 10
let y: Int = 5
let z: Int = (x + y) * 2 - 3      // 27

let a: Float = 2.0
let b: Float = 3.0
let c: Float = a ^ b + 1.0        // 9.0 (2^3 = 8, then +1)
```

## Examples

```vyn
// Calculating area
let width: Int = 12
let height: Int = 8
let area: Int = width * height
stdout# area  // 96

// Temperature conversion (Celsius to Fahrenheit)
let celsius: Float = 25.0
let fahrenheit: Float = celsius * 9.0 / 5.0 + 32.0
stdout# fahrenheit  // 77.0

// Checking conditions
let age: Int = 18
let is_adult: Bool = age >= 18
stdout# is_adult  // true

// Power calculation
let base: Int = 3
let exponent: Int = 4
let result: Int = base ^ exponent
stdout# result  // 81

// Working with arrays
let scores: [3]Int = [85, 92, 78]
let coordinates: [2]Float = [3.5, 7.2]
stdout# scores
stdout# coordinates
```

## Type Compatibility

Expressions must use compatible types:

```vyn
let x: Int = 5 + 3        // OK - both Int
let y: Float = 2.5 + 1.5  // OK - both Float
let z: Int = 5 + 2.5      // Error! Cannot mix Int and Float

let arr: [2]Int = [1, 2]  // OK - correct size
let bad: [2]Int = [1, 2, 3]  // Error! Size mismatch
```

## Next Steps

Learn about [Operators](operators.md) for a complete reference of Vyn's operators and their precedence.
