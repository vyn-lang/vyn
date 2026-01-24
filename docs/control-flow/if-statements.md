# If Statements

If statements let your program make decisions by executing code only when certain conditions are true.

## Basic If Statement

The simplest form checks a condition and executes code if it's true:

```vyn
if true {
    stdout# "This will print"
}
```

## If-Else

Use `else` to specify code that runs when the condition is false:

```vyn
let age: Int = 18

if age >= 18 {
    stdout# "Adult"
} else {
    stdout# "Minor"
}
```

## Syntax

```vyn
if <condition> {
    // code to run if condition is true
} else {
    // code to run if condition is false
}
```

The `else` block is optional:

```vyn
if age >= 18 {
    stdout# "You can vote"
}
// No else needed
```

## Else-If Chains

Check multiple conditions using `else if`:

```vyn
let score: Int = 85

if score >= 90 {
    stdout# "A"
} else if score >= 80 {
    stdout# "B"
} else if score >= 70 {
    stdout# "C"
} else {
    stdout# "F"
}
```

## Truthy Values

Conditions can be any expression that evaluates to a truthy value. In Vyn, any boolean expression works:

```vyn
let is_active: Bool = true

if is_active {
    stdout# "System is active"
}
```

```vyn
let count: Int = 5

if count > 0 {
    stdout# "We have items"
}
```

## Examples

### Simple Condition

```vyn
let temperature: Int = 40

if temperature > 70 {
    stdout# "It's hot outside"
} else {
    stdout# "It's cool outside"
}
```

### Multiple Conditions

```vyn
let age: Int = 25
let hasLicense: Bool = true

if age >= 18 {
    if hasLicense {
        stdout# "Can drive"
    } else {
        stdout# "Need a license"
    }
} else {
    stdout# "Too young to drive"
}
```

### Else-If Chain

```vyn
let hour: Int = 14

if hour < 12 {
    stdout# "Good morning"
} else if hour < 18 {
    stdout# "Good afternoon"
} else {
    stdout# "Good evening"
}
```

### With Variables

```vyn
let x: Int = 10
let y: Int = 20
let @max: Int = 0

if x > y {
    max = x
} else {
    max = y
}

stdout# max  // 20
```

### Numeric Comparisons

```vyn
let price: Float = 99.99
let budget: Float = 100.0

if price <= budget {
    stdout# "You can afford this"
} else {
    stdout# "Too expensive"
}
```

### Boolean Checks

```vyn
let isLoggedIn: Bool = true
let hasPermission: Bool = false

if isLoggedIn {
    if hasPermission {
        stdout# "Access granted"
    } else {
        stdout# "Access denied"
    }
} else {
    stdout# "Please log in"
}
```

## Nested If Statements

You can nest if statements inside each other:

```vyn
let age: Int = 20
let country: String = "US"

if age >= 18 {
    if country == "US" {
        stdout# "Can vote in US elections"
    } else {
        stdout# "Can vote in other countries"
    }
} else {
    stdout# "Too young to vote"
}
```

## Common Patterns

### Finding Maximum

```vyn
let a: Int = 15
let b: Int = 23
let @result: Int = 0

if a > b {
    result = a
} else {
    result = b
}

stdout# result
```

### Range Checking

```vyn
let value: Int = 50

if value >= 0 {
    if value <= 100 {
        stdout# "In range"
    } else {
        stdout# "Too high"
    }
} else {
    stdout# "Too low"
}
```

### Status Classification

```vyn
let status: Int = 200

if status == 200 {
    stdout# "OK"
} else if status == 404 {
    stdout# "Not Found"
} else if status == 500 {
    stdout# "Server Error"
} else {
    stdout# "Unknown Status"
}
```

## Best Practices

### Use Meaningful Conditions

```vyn
// Good - clear intent
let can_purchase: Bool = age >= 21
if can_purchase {
    stdout# "Sale approved"
}

// Less clear
if age >= 21 {
    stdout# "Sale approved"
}
```

### Avoid Deep Nesting

```vyn
// Hard to read
if condition1 {
    if condition2 {
        if condition3 {
            // deeply nested
        }
    }
}

// Better - use else-if when possible
if condition1 {
    // code
} else if condition2 {
    // code
} else if condition3 {
    // code
}
```

### Keep Conditions Simple

```vyn
// Complex
if (x > 0) {
    if (x < 100) {
        if (x != 50) {
            stdout# "Valid"
        }
    }
}

// Clearer with variables
let isPositive: Bool = x > 0
let isInRange: Bool = x < 100
let isNotFifty: Bool = x != 50

if isPositive {
    if isInRange {
        if isNotFifty {
            stdout# "Valid"
        }
    }
}
```

## Next Steps

More control flow features are coming soon, including:

- Loops (while, for)
- Match statements
- Break and continue

For now, you can combine if statements with the [Basics](../basics/index.md) you've learned to create powerful conditional logic!
