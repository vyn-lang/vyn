# Type Aliasing

Type aliases let you create custom names for existing types. This makes your code more readable and expressive.

## Creating Type Aliases

Use the `type` keyword to create an alias:

```vyn
type UserId = Int
```

Now `UserId` can be used anywhere you would use `Int`.

## Syntax

```vyn
type <name> = <type>
```

Semicolons are optional:

```vyn
type UserId = Int;
type Score = Float;
```

Or without semicolons:

```vyn
type UserId = Int
type Score = Float
```

## Using Type Aliases

Once defined, use the alias like any other type:

```vyn
type Age = Int

let userAge: Age = 25
let siblingAge: Age = 30
```

## Why Use Type Aliases?

Type aliases make code more self-documenting:

```vyn
// Without aliases - less clear
let id: Int = 12345
let count: Int = 100

// With aliases - more meaningful
type ProductId = Int
type Quantity = Int

let id: ProductId = 12345
let count: Quantity = 100
```

## Examples

```vyn
// Define aliases
type Distance = Float
type Temperature = Float
type Username = String

// Use them
let miles: Distance = 26.2
let celsius: Temperature = 20.5
let user: Username = "alice"

stdout# miles
stdout# celsius
stdout# user
```

```vyn
// Multiple related aliases
type Meters = Float
type Seconds = Float
type Speed = Float

let distance: Meters = 100.0
let time: Seconds = 9.58
let @speed: Speed = 0.0
speed = distance / time

stdout# speed
```

## Best Practices

- Use meaningful names that describe what the value represents
- Group related type aliases together
- Prefer aliases over primitive types for domain-specific values

## Important Note

Type aliases are just alternate names - they don't create new types. A `UserId` is still an `Int`:

```vyn
type UserId = Int
type ProductId = Int

let user: UserId = 123
let product: ProductId = user  // This works - both are Int
```

## Next Steps

Learn about [Expressions](expressions.md) to understand how to compute and combine values.
