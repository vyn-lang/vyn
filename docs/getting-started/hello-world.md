# Hello World

Let's write your first Vyn program!

## Your First Program

Create a new file called `hello.vyn`:

```vyn
stdout# "Hello, World!"
```

That's it! In Vyn, `stdout#` prints an expression to standard output.

## Running Your Program

Run your program using the Vyn CLI:

```bash
vyn run hello.vyn
```

You should see:

```
Hello, World!
```

## How It Works

- `stdout#` - Outputs an expression to the console
- `"Hello, World!"` - A string expression

## Try It Yourself

Try printing different things:

```vyn
stdout# "Welcome to Vyn!"
stdout# 42
stdout# 3.14
```

## Next Steps

Now that you can run programs, learn more about the [CLI Usage](cli-usage.md) to explore other Vyn commands.
