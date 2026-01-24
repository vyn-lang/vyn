# Installation

## Installing from Source

Currently, Vyn must be built from source.

### Prerequisites

- **Rust** (latest stable version)
- **Cargo** (comes with Rust)

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/).

### Building Vyn

1. Clone the repository:

   ```bash
   git clone https://github.com/vyn-lang/vyn.git
   cd vyn
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The compiled binary will be at `target/release/vyn`

4. (Optional) Install globally:
   ```bash
   cargo install --path .
   ```

### Verify Installation

Check that Vyn is installed correctly:

```bash
vyn version
```

You should see version information displayed.

## Next Steps

Now that Vyn is installed, let's write your first program! Continue to [Hello World](hello-world.md).
