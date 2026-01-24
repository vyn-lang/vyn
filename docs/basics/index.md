# Basics

Learn the fundamental building blocks of Vyn. This section covers variables, types, expressions, and the core syntax you'll use every day.

## In This Section

- **[Variables](variables.md)** - Declaring and using variables
- **[Data Types](data-types.md)** - Understanding Vyn's type system
- **[Type Aliasing](type-aliasing.md)** - Creating custom type names
- **[Expressions](expressions.md)** - Working with values and computations
- **[Operators](operators.md)** - Arithmetic, comparison, and logical operations
- **[Comments](comments.md)** - Documenting your code

## Quick Overview

Here's a taste of Vyn's basic syntax:

```vyn
// Variables are immutable by default
let x: Int = 5

// Mutable variables use @ prefix
let @count: Int = 0
count = 10

// Type aliasing
type UserId = Int

// Expressions and operators
let result: Int = (10 + 5) * 2
let power: Int = 2 ^ 8

// Output
stdout# result
```

## Key Concepts

- **Immutability by default** - Variables are immutable unless marked with `@`
- **Static typing** - All types are checked at compile time
- **Type inference** - Coming soon
- **Optional semicolons** - Use them for clarity or omit them for brevity

## Next Steps

Start with [Variables](variables.md) to learn how to store and manipulate data in Vyn.
