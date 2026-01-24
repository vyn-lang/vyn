# Data Types

Vyn is a statically typed language, meaning every variable has a type known at compile time. This section covers Vyn's primitive types.

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

## Type Annotations

Every variable must specify its type:

```vyn
let count: Int = 0
let score: Float = 98.5
let is_valid: Bool = true
let username: String = "alice"
```

## Examples

```vyn
// Different types
let quantity: Int = 100
let discount: Float = 0.15
let in_stock: Bool = true
let product_name: String = "Widget"

// Using types in expressions
let total: Int = quantity * 2
let final_price: Float = 50.0 * (1.0 - discount)

// Output
stdout# productName
stdout# total
```

## Type Safety

Vyn's type system prevents many common errors at compile time:

```vyn
let x: Int = 5
let y: String = "10"
let z: Int = x + y  // Error! Cannot add Int and String
```

The compiler catches type mismatches before your program runs.

## Next Steps

Learn about [Type Aliasing](type-aliasing.md) to create custom names for types.
