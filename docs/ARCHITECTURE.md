# LEXOR Interpreter Architecture

This interpreter is built entirely in Rust and follows the classic 3-stage interpretation model:

1. **Lexical Analysis (Scanner / Lexer)**
2. **Syntax Analysis (Parser)**
3. **Evaluation (Interpreter)**

## 1. The Lexer (`src/lexer.rs` & `src/tokens.rs`)
The Lexer's job is to read raw Source Code characters (like `D` `E` `C` `L` `A` `R` `E`) and group them into logical, structured "Tokens".
- `Token` definitions are stored tightly in `src/tokens.rs` as a Rust `enum`.
- The `Lexer` struct acts as a native Rust `Iterator`. You can loop over a Lexer stream safely using `for token in lexer`.
- *Error Handling:* Any unknown characters or malformed primitives are packaged harmlessly into a `Token::Illegal` variant rather than crashing the program.

## 2. The Parser (`src/parser.rs` & `src/ast.rs`)
The Parser's job is to consume the stream of Tokens and build an **Abstract Syntax Tree (AST)** structurally defining what the code *means*. 
- `ast.rs` heavily utilizes Rust Enums (the `Statement` enum and `Expression` enum) to guarantee at compile-time that every node type is strictly matched.
- The Lexer generates a 1D list of tokens. The Parser generates a 3D tree of AST nodes.

## 3. The Evaluator (Implementation Pending)
The Evaluator walks through the finalized AST tree recursively, maintaining a "Global Environment" HashMap (to track declared variables) and actually executing the side-effects like printing to the screen or evaluating mathematics.
