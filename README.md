# LEXOR Programming Language

A tree-walking interpreter for the LEXOR programming language, implemented in Rust. It parses source files and executes them using a standard Abstract Syntax Tree (AST) approach.

## Usage
The project is structured as a Cargo Workspace. You can run the interpreter through the CLI:

```bash
cargo run --package lexor_cli -- samples/fibonacci.lexor
```

You can also use the WebAssembly bindings in a browser environment by building the `wasm` crate.

## Project Structure
The code is split into three main parts:

### 1. `core/` (The Interpreter)
Contains the core logic for the language.
- `lexer.rs`: Breaks source text into tokens.
- `parser.rs`: Converts tokens into an AST (uses Pratt parsing for math).
- `ast.rs`: Definitions for the tree nodes.
- `evaluator.rs`: Walks the tree to execute the code.
- `object.rs`: The internal value types (Integers, Floats, Booleans, etc.).

### 2. `cli/` (CLI Tool)
A simple wrapper that provides terminal input/output for the interpreter.

### 3. `wasm/` (Web/JS Support)
Provides bindings to run the interpreter in a browser or Node.js environment via WebAssembly.

## Main Features
- **Pratt Parsing:** Handles math order of operations (precedence) correctly.
- **Control Flow:** Supports basic `IF`, `ELSE`, `FOR`, and `REPEAT WHEN` blocks.
- **Type Checking:** Simple runtime checks for variable assignments.
- **Environment:** A basic memory store using HashMaps to track variables during execution.

## Testing & Local Checks
To check formatting, run tests, and verify lints locally, you can use the provided scripts:

**Windows PowerShell:**
```powershell
.\scripts\check.ps1
```

**Linux / macOS:**
```bash
./scripts/check.sh
```

*(Note: These scripts require `cargo-deny` to be installed for license/security checks: `cargo install --locked cargo-deny`)*

## Documentation
Additional details on how the lexer, parser, and evaluator were built can be found in the `docs/` folder.
