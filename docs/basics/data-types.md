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

## Collection Types

### Arrays

Arrays have a fixed size specified in the type annotation using the syntax `[n]T`, where `n` is the size and `T` is the element type. Array literals must contain exactly the declared number of elements.

```vyn
let coordinates: [3]Float = [1.5, 2.7, 3.9]
let flags: [4]Bool = [true, false, true, false]
let names: [2]String = ["Alice", "Bob"]

// Compiler enforces size
let valid: [3]Int = [1, 2, 3]
let invalid: [3]Int = [1, 2]         // Error: Expected 3 elements
```

### Sequences (Dynamic Arrays)

Sequences are dynamic arrays that can grow and shrink at runtime. The syntax is `[]T`, where `T` is the element type. Sequences can hold any number of elements, including zero.

```vyn
let numbers: []Int = [1, 2, 3, 4, 5]
let scores: []Float = [95.5, 87.3, 92.1]
let tags: []String = ["vyn", "programming", "language"]
let empty: []Int = []
```

## Type Annotations

All variables require explicit type annotations:

```vyn
let count: Int = 0
let is_valid: Bool = true
let points: [5]Int = [10, 20, 30, 40, 50]
let dynamic_points: []Int = [10, 20, 30]
```

## Type Safety

The compiler enforces type correctness at compile time:

```vyn
let x: Int = 5
let y: String = "10"
let z: Int = x + y              // Error: Cannot add Int and String

let arr: [2]Int = [1, 2, 3]     // Error: Size mismatch
let seq: []String = [1, 2, 3]   // Error: Type mismatch (Int vs String)
```

## Next Steps

Learn about [Type Aliasing](type-aliasing.md) to create custom names for types.
