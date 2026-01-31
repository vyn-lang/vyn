# Vyn Programming Language

**Vyn** is a statically typed programming language that compiles to portable bytecode and executes on a custom virtual machine.  
The Vyn programming language is designed to provide consistent behavior across platforms by separating compilation from execution through a well-defined bytecode format and runtime.

Vyn is currently under active development and focuses on building a clear language specification, a predictable execution model, and a maintainable compiler architecture.

---

## Overview

The Vyn programming language follows a virtual machine–based design.  
Source code written in Vyn is compiled into `.vync` bytecode files, which are then executed by the Vyn runtime. This approach allows the language to remain portable while maintaining explicit control over execution semantics.

Vyn is an independent programming language and is not related to the V programming language (Vlang). It uses its own syntax, compiler, bytecode format, and virtual machine implementation.

---

## Design Goals

Vyn is developed with the following goals in mind:

- **Static Typing**  
  The language uses a static type system to detect errors during compilation and enforce predictable program behavior.

- **Portable Bytecode**  
  Vyn programs compile to a platform-independent bytecode format (`.vync`), allowing the same compiled output to run on any supported system.

- **Virtual Machine Execution**  
  Execution is handled by a custom virtual machine designed to be explicit, debuggable, and suitable for tooling and experimentation.

- **Clear and Explicit Syntax**  
  Language features prioritize readability and explicit control flow over implicit or hidden behavior.

- **Efficient Runtime**  
  The runtime is designed to execute bytecode with low overhead while remaining simple to reason about and extend.

---

## Architecture

The Vyn programming language is composed of several core components:

1. **Compiler**  
   Translates a valid Vyn source code into validated bytecode.

2. **Bytecode Format**  
   A structured, portable representation of programs designed for consistent execution across platforms.

3. **Virtual Machine**  
   Executes Vyn bytecode and defines the language’s runtime semantics.

4. **Runtime Environment**  
   Provides standard facilities required during execution, including I/O and memory management.

This architecture allows the language to evolve independently at the source, bytecode, and runtime levels.

---

## Current Status

⚠️ **Early Development**

Vyn is in an early stage of development.  
Language features, bytecode instructions, and runtime behavior are subject to change as the design matures.

Documentation and tooling are being expanded alongside core language development.

---

## Documentation

Project documentation covers:

- Installation and setup
- Language syntax and semantics
- Compiler behavior
- Virtual machine design
- Runtime architecture

📄 See the full documentation: **[docs/index.md](docs/index.md)**

---

## Building from Source

Instructions for building the Vyn compiler and runtime from source are available in the installation guide:

➡️ **[Getting Started – Installation](docs/getting-started/installation.md)**

---

## Contributing

Vyn is an experimental programming language in active development.  
Contributions in the form of bug reports, design discussions, and code improvements are welcome.

Please review **[CONTRIBUTING.md](CONTRIBUTING.md)** before submitting changes.

---

## License

The Vyn programming language is released under the **MIT License**.  
See **[LICENSE](LICENSE)** for licensing details.

---

<div align="center">
  Developed by the <a href="https://github.com/vyn-lang">Vyn Team</a>
</div>
