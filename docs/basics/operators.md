# Operators

Vyn provides several operators for working with values. This page covers all available operators and their usage.

## Arithmetic Operators

### Addition (`+`)

Adds two numbers:

```vyn
let sum: Int = 5 + 3        // 8
let total: Float = 2.5 + 1.5  // 4.0
```

### Subtraction (`-`)

Subtracts one number from another:

```vyn
let difference: Int = 10 - 4     // 6
let result: Float = 5.5 - 2.5    // 3.0
```

### Multiplication (`*`)

Multiplies two numbers:

```vyn
let product: Int = 6 * 7         // 42
let area: Float = 3.5 * 2.0      // 7.0
```

### Division (`/`)

Divides one number by another:

```vyn
let quotient: Float = 15.0 / 3.0   // 5.0
let half: Float = 10.0 / 2.0       // 5.0
```

### Exponentiation (`^`)

Raises a number to a power:

```vyn
let squared: Int = 5 ^ 2      // 25
let cubed: Int = 2 ^ 3        // 8
let power: Int = 10 ^ 4       // 10000
```

## Indexing Operator

### Index (`::`)

Access elements in an array using the box colon operator (also called the index operator):

```vyn
let numbers: [3]Int = [10, 20, 30]
let first: Int = numbers::0      // 10
let second: Int = numbers::1     // 20
let third: Int = numbers::2      // 30
```

The syntax is `target::index` where:

- `target` is the array expression
- `index` is the zero-based position

```vyn
let scores: [5]Int = [85, 92, 78, 95, 88]
let best: Int = scores::3        // 95
let worst: Int = scores::2       // 78

let coordinates: [2]Float = [3.5, 7.2]
let x: Float = coordinates::0    // 3.5
let y: Float = coordinates::1    // 7.2

let names: [3]String = ["Alice", "Bob", "Charlie"]
let name: String = names::1      // "Bob"
```

## Comparison Operators

### Equal (`==`)

Checks if two values are equal:

```vyn
let is_equal: Bool = 5 == 5       // true
let same: Bool = 3 == 7          // false
```

### Not Equal (`!=`)

Checks if two values are different:

```vyn
let is_different: Bool = 5 != 3   // true
let not_same: Bool = 4 != 4       // false
```

### Greater Than (`>`)

Checks if left value is greater than right:

```vyn
let is_greater: Bool = 10 > 5     // true
let bigger: Bool = 3 > 7         // false
```

### Less Than (`<`)

Checks if left value is less than right:

```vyn
let is_less: Bool = 2 < 8         // true
let smaller: Bool = 9 < 4        // false
```

### Greater Than or Equal (`>=`)

Checks if left value is greater than or equal to right:

```vyn
let check1: Bool = 5 >= 5        // true
let check2: Bool = 7 >= 3        // true
let check3: Bool = 2 >= 9        // false
```

### Less Than or Equal (`<=`)

Checks if left value is less than or equal to right:

```vyn
let check1: Bool = 3 <= 5        // true
let check2: Bool = 4 <= 4        // true
let check3: Bool = 8 <= 2        // false
```

## Operator Precedence

Operators are evaluated in this order (highest to lowest):

1. **Parentheses** `()`
2. **Array Indexing** `::`
3. **Exponentiation** `^`
4. **Multiplication and Division** `*` `/`
5. **Addition and Subtraction** `+` `-`
6. **Comparison** `==` `!=` `>` `<` `>=` `<=`

### Examples

```vyn
let result1: Int = 2 + 3 * 4       // 14 (3*4=12, then 2+12)
let result2: Int = (2 + 3) * 4     // 20 (2+3=5, then 5*4)
let result3: Int = 2 ^ 3 + 1       // 9 (2^3=8, then 8+1)
let result4: Int = 2 + 3 ^ 2       // 11 (3^2=9, then 2+9)

let arr: [3]Int = [1, 2, 3]
let indexed: Int = arr::1 + 5      // 7 (arr::1=2, then 2+5)
```

## Using Operators

```vyn
// Arithmetic
let x: Int = 10
let y: Int = 3
let sum: Int = x + y         // 13
let product: Int = x * y     // 30
let power: Int = x ^ 2       // 100

// Array indexing
let values: [4]Int = [100, 200, 300, 400]
let first: Int = values::0         // 100
let last: Int = values::3          // 400

// Comparisons
let is_greater: Bool = x > y       // true
let is_equal: Bool = x == y        // false

// Complex expressions
let @counter: Int = 0
counter = counter + 1
counter = counter * 2
stdout# counter  // 2

// Combining operators
let a: Int = 5
let b: Int = 3
let c: Int = 2
let result: Int = (a + b) * c ^ 2
stdout# result  // 32 ((5+3) * 2^2 = 8 * 4)

// Indexing with expressions
let data: [5]Int = [10, 20, 30, 40, 50]
let index: Int = 2
let element: Int = data::index     // 30
let doubled: Int = data::1 * 2     // 40 (20 * 2)
```

## Type Requirements

- Arithmetic operators work with `Int` and `Float`
- Comparison operators work with `Int`, `Float`, `Bool`, and `String`
- The indexing operator works with arrays and requires an `Int` index
- Both operands must have the same type for arithmetic and comparison operations

```vyn
let valid1: Int = 5 + 3           // OK - both Int
let valid2: Float = 2.5 + 1.5     // OK - both Float
let valid3: Bool = 5 > 3          // OK - comparing Int

let arr: [3]Int = [1, 2, 3]
let valid4: Int = arr::0          // OK - valid index

let invalid: Int = 5 + 2.5        // Error! Cannot mix Int and Float
```

## Next Steps
