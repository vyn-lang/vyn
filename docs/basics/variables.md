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
// Immutable variables
let pi: Float = 3.14159
let greeting: String = "Hello"

// Mutable counter
let @counter: Int = 0
counter = counter + 1
counter = counter + 1
stdout# counter  // Outputs: 2

// Multiple variables
let width: Int = 100
let height: Int = 50
let area: Int = width * height
```

## Best Practices

- Use immutable variables by default
- Only use `@` when you actually need to modify a variable
- Choose descriptive variable names
- Group related variable declarations together

## Next Steps

Learn about Vyn's [Data Types](data-types.md) to understand what kinds of values you can store in variables.
