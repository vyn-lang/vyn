# Data Types

Vyn is a statically typed language, meaning every variable has a type known at compile time. This section covers Vyn's primitive types and non-primitive types' syntax.

## Primitive Types

Vyn's primitive types are written in PascalCase:

### Int

Integers are whole numbers without a decimal point:

```vyn
let age: Int = 25
let year: Int = 2026
let negative: Int = -42
```

### Float

Floating-point numbers represent decimal values:

```vyn
let pi: Float = 3.14159
let price: Float = 19.99
let temperature: Float = -40.5
```

### Bool

Booleans represent true or false values:

```vyn
let is_active: Bool = true
let has_error: Bool = false
```

### String

Strings represent text:

```vyn
let name: String = "Vyn"
let message: String = "Hello, World!"
let empty: String = ""
```

## Fixed-Size Arrays

Arrays in Vyn have a fixed size that must be specified in the type annotation. The syntax is `[n]T` where `n` is the size and `T` is the element type.

The array contents **must** be exactly the specified size—no more, no less.

```vyn
let arr: [2]Int = [10, 20]
let coordinates: [3]Float = [1.5, 2.7, 3.9]
let flags: [4]Bool = [true, false, true, false]
let names: [2]String = ["Alice", "Bob"]
```

### Array Size Enforcement

The compiler enforces that array literals match the declared size:

```vyn
let valid: [3]Int = [1, 2, 3]        // ✓ Correct
let too_few: [3]Int = [1, 2]         // ✗ Error! Expected 3 elements
let too_many: [3]Int = [1, 2, 3, 4]  // ✗ Error! Expected 3 elements
```

## Type Annotations

Every variable must specify its type:

```vyn
let count: Int = 0
let score: Float = 98.5
let is_valid: Bool = true
let username: String = "alice"
let points: [5]Int = [10, 20, 30, 40, 50]
```

## Examples

```vyn
// Different types
let quantity: Int = 100
let discount: Float = 0.15
let in_stock: Bool = true
let product_name: String = "Widget"

// Fixed-size arrays
let inventory: [3]Int = [50, 75, 100]
let prices: [2]Float = [19.99, 29.99]

// Using types in expressions
let total: Int = quantity * 2
let final_price: Float = 50.0 * (1.0 - discount)

// Output
stdout# product_name
stdout# total
stdout# inventory
```

## Type Safety

Vyn's type system prevents many common errors at compile time:

```vyn
let x: Int = 5
let y: String = "10"
let z: Int = x + y  // Error! Cannot add Int and String

let arr1: [2]Int = [1, 2, 3]  // Error! Size mismatch
```

The compiler catches type mismatches before your program runs.

## Next Steps

Learn about [Type Aliasing](type-aliasing.md) to create custom names for types.
