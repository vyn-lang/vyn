# Variables

Variables in Vyn store data that your program can use and manipulate. Vyn enforces immutability by default, making your code safer and easier to reason about.

## Declaring Variables

Use the `let` keyword to declare a variable:

```vyn
let x: Int = 5
```

Breaking this down:

- `let` - Declares a new variable
- `x` - The variable name
- `: Int` - The type annotation
- `= 5` - The initial value

## Immutability by Default

Variables in Vyn are **immutable** by default. Once assigned, their value cannot change:

```vyn
let x: Int = 5
x = 10  // Error! Cannot mutate immutable variable
```

This prevents accidental modifications and makes code more predictable.

## Mutable Variables

To create a mutable variable, prefix the name with `@`:

```vyn
let @x: Int = 5
x = 10  // OK! Variable is mutable
x = 15  // Also OK
```

The `@` symbol indicates that this variable's value can change over time.

### Mutating Array Elements

Mutable variables are required when modifying array elements:

```vyn
let arr: [3]Int = [1, 2, 3]
arr::1 = 10  // Error! arr is immutable

let @mut_arr: [3]Int = [1, 2, 3]
mut_arr::1 = 10  // OK! mut_arr is mutable
stdout# mut_arr  // Outputs: [1, 10, 3]
```

## Static Variables

Static variables are compile-time constants that never change. They're declared with the `static` keyword and are often used for configuration values or array sizes:

```vyn
static MAX_USERS: Int = 100
static PI: Float = 3.14159
static APP_NAME: String = "MyApp"
```

Key properties of static variables:

- **Compile-time evaluation**: Their values must be known at compile time
- **Cannot be mutated**: Static variables are always immutable
- **Used in type annotations**: Static integers can be used to define array sizes

```vyn
static ARRAY_SIZE: Int = 5
let numbers: [ARRAY_SIZE]Int = [1, 2, 3, 4, 5]
```

Attempting to mutate a static variable results in an error:

```vyn
static MAX_SCORE: Int = 100
MAX_SCORE = 200  // Error! Cannot mutate static variable
```

## Type Annotations

All variables must have a type annotation:

```vyn
let name: String = "Vyn"
let count: Int = 42
let price: Float = 19.99
let active: Bool = true
```

## Semicolons

Semicolons are optional in Vyn. These are equivalent:

```vyn
let x: Int = 5;
let y: Int = 10;
```

```vyn
let x: Int = 5
let y: Int = 10
```

Use semicolons for clarity or omit them for cleaner code - it's your choice!

## Examples

```vyn
// Static constants
static MAX_CONNECTIONS: Int = 1000
static DEFAULT_TIMEOUT: Float = 30.0

// Immutable variables
let pi: Float = 3.14159
let greeting: String = "Hello"

// Mutable counter
let @counter: Int = 0
counter = counter + 1
counter = counter + 1
stdout# counter  // Outputs: 2

// Array with static size
static BUFFER_SIZE: Int = 256
let @buffer: [BUFFER_SIZE]Int = [0, 0, 0, ...]
buffer::0 = 42

// Multiple variables
let width: Int = 100
let height: Int = 50
let area: Int = width * height
```

## Best Practices

- Use `static` for compile-time constants that never change
- Use immutable variables (`let`) by default for runtime values
- Only use `@` when you actually need to modify a variable
- Choose descriptive variable names
- Group related variable declarations together
- Use static variables for array sizes to make code more maintainable

## Next Steps

Learn about Vyn's [Data Types](data-types.md) to understand what kinds of values you can store in variables.
