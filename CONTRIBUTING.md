# Contributing to Vyn

Thank you for your interest in contributing to Vyn! This document provides guidelines for contributing to the project.

## Current Status

Vyn is in early development. While we welcome contributions, please note that the language design and implementation are still evolving rapidly.

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior vs actual behavior
- Your environment (OS, Vyn version, etc.)
- Any relevant code samples or error messages

### Suggesting Features

We welcome feature suggestions! Please open an issue with:

- A clear description of the feature
- Why it would be useful for Vyn
- Any examples or use cases
- How it might work (if you have ideas)

### Code Contributions

Before starting work on a significant change:

1. Check existing issues and PRs to avoid duplicate work
2. Open an issue to discuss your proposed changes
3. Wait for feedback from maintainers

#### Pull Request Process

1. Fork the repository
2. Create a new branch for your feature (`git checkout -b feature/your-feature-name`)
3. Make your changes
4. Test your changes thoroughly
5. Commit with clear, descriptive messages
6. Push to your fork
7. Open a pull request with:
   - A clear description of the changes
   - Any related issue numbers
   - Screenshots or examples if applicable

### Documentation

Documentation improvements are always welcome! This includes:

- Fixing typos or clarifying existing docs
- Adding examples
- Writing tutorials
- Improving code comments

## Development Setup

### Prerequisites

- **Rust** (latest stable version)
- **Cargo** (comes with Rust)

### Building from Source

1. Clone the repository:

   ```bash
   git clone https://github.com/vyn-lang/vyn.git
   cd vyn
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. The compiled binary will be available at `target/release/vyn`

4. (Optional) Install locally:
   ```bash
   cargo install --path .
   ```

### Running Vyn

The Vyn CLI provides several commands:

```bash
# Run a Vyn program
vyn run program.vyn

# Type check without running
vyn check program.vyn

# Disassemble bytecode
vyn disasm program.vync

# Show version
vyn version
```

### CLI Options

- `--no-progress` - Disable progress bar
- `--verbose` - Show timing information for each phase
- `-q, --quiet` - Minimal output
- `--slow-mode` - Slow down compilation for debugging
- `--time` - Show time taken for each phase

### Development Tips

- Use `cargo run -- <command>` to run Vyn without installing
- Use `--verbose` or `--time` flags to debug performance issues
- Use `--slow-mode` to step through compilation phases

## Code Style

- Write clear, readable code
- Comment complex logic
- Follow existing code patterns in the project
- Keep commits focused and atomic
- Follow Rust naming conventions and idioms

## Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Questions?

If you have questions about contributing, feel free to:

- Open a discussion on GitHub
- Comment on relevant issues

## License

By contributing to Vyn, you agree that your contributions will be licensed under the MIT License.

---

Thank you for helping make Vyn better ❤️
