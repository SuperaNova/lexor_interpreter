# LEXOR Programming Language Engine

A perfectly crafted tree-walking evaluation macro language and execution engine built entirely in modern, idiomatic Rust. It parses source files safely and dynamically structures them into powerful Abstract Syntax Trees.

## Usage
Simply invoke the interpreter natively by pointing it sequentially toward any `.lexor` scripts:
```bash
cargo run -- samples/fibonacci.lexor
```

## Features
- **Pratt Parsing Engine:** Enforces perfect algorithmic Order of Operations for deep mathematics and safely manages right-associative chained variables dynamically.
- **Top-Down Recursive Descent:** Strictly guarantees clean execution of nested `IF`, `ELSE`, `FOR`, and `REPEAT WHEN` control block validations iteratively.
- **Safely Typed Evaluator:** The Execution Engine structurally evaluates mathematics but fails gracefully before panicking by explicitly bubbling `Object::Error` variants immediately upon logic failures (like dividing by zero).
- **RAM Environment State:** Employs standard Rust HashMaps linking assignments indefinitely securely over runtime iteration states identically to modern scripting languages.

## Project Structure
This structure fundamentally complies with classic compilation engine milestones:
- `src/lexer.rs`: The Lexical Analyzer (Scanner).
- `src/parser.rs`: The Syntax Analyzer mapping Tokens natively to AST shapes.
- `src/ast.rs`: The 3D Enum structure definitions statically representing the Tree. 
- `src/evaluator.rs`: The execution logic safely walking AST nodes into actions.
- `src/object.rs`: The unified generic typed wrappers for all Lexor variables (`String`, `Integer`, `Null`).

## Architecture & Maintenance
See the `docs/` folder for deeply structural documentation concerning exactly how `lexer.rs` behaves, and uniquely how the `parser.rs` Pratt Algorithm tokenizes precedence correctly over expressions!