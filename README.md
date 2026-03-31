# LEXOR Programming Language Engine

A perfectly crafted tree-walking evaluation macro language and execution engine built entirely in modern, idiomatic Rust. It parses source files safely and dynamically structures them into powerful Abstract Syntax Trees.

## Usage
Since LEXOR is built as a highly modular Cargo Workspace, you can run the interpreter natively from the CLI:

```bash
cargo run --package lexor_cli -- samples/fibonacci.lexor
```

Or, you can use the WebAssembly bindings directly in any JavaScript web frontend by building the `wasm` crate!

## Workspace Structure
This structure fundamentally complies with classic compilation engine milestones, decoupled across three distinct crates:

### 1. `core/` (The Engine)
The heavily isolated pure logic engine.
- `core/src/lexer.rs`: The Lexical Analyzer (Scanner).
- `core/src/parser.rs`: The Syntax Analyzer mapping Tokens natively to AST shapes.
- `core/src/ast.rs`: The 3D Enum structure definitions statically representing the Tree. 
- `core/src/evaluator.rs`: The execution logic safely walking AST nodes into actions.
- `core/src/object.rs`: The unified generic typed wrappers for all Lexor variables (`String`, `Integer`, `Null`).

### 2. `cli/` (Terminal Executor)
Provides the standard Terminal string input/output implementations for the engine and acts as a runnable `.exe`.

### 3. `wasm/` (WebAssembly Bridge)
Injects Mock inputs and strings capturing to securely run `core` in web browsers without freezing the Javascript thread.

## Features
- **Pratt Parsing Engine:** Enforces perfect algorithmic Order of Operations for deep mathematics and safely manages right-associative chained variables dynamically.
- **Top-Down Recursive Descent:** Strictly guarantees clean execution of nested `IF`, `ELSE`, `FOR`, and `REPEAT WHEN` control block validations iteratively.
- **Safely Typed Evaluator:** The Execution Engine structurally evaluates mathematics but fails gracefully before panicking by explicitly bubbling `Object::Error` variants immediately upon logic failures (like dividing by zero).
- **RAM Environment State:** Employs standard Rust HashMaps linking assignments indefinitely securely over runtime iteration states identically to modern scripting languages.

## Developer Utilities & Local Checking
To securely check code locally before pushing to prevent messy CI/CD failures (like Cargo Deny license rejections or Test panic tracebacks), run this quick check locally:

```bash
# 1. Run all tests locally across the entire workspace
cargo test --workspace

# 2. Assert no vulnerable dependencies or banned licenses locally
# (Note: Requires installing `cargo-deny` first: `cargo install --locked cargo-deny`)
cargo deny check
```

## Architecture & Maintenance
See the `docs/` folder for deeply structural documentation concerning exactly how `lexer.rs` behaves, and uniquely how the `parser.rs` Pratt Algorithm tokenizes precedence correctly over expressions!