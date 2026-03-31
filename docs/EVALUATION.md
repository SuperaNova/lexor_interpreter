# LEXOR Evaluator

The Evaluator is responsible for executing the structured AST produced by the parser. The logic is defined in `src/evaluator.rs`.

## Runtime Objects (`object.rs`)
The Evaluator uses the `Object` enum to represent variables dynamically during runtime.
- Includes strict variants like `Integer(i32)`, `Float(f32)`, `String(String)`, and `Boolean(bool)`.
- Runtime issues are returned as `Object::Error(String)` which safely stops execution instead of panicking.

## Variable Memory (`environment.rs`)
Variables initialized with `DECLARE` are actively stored in the `Environment` struct. The environment acts as localized memory (wrapping a standard Rust `HashMap<String, Object>`), allowing the evaluator to look up existing variables like `x` when performing calculations.

## Tree-Walking Execution (`evaluator.rs`)
The `eval_program()` function walks through the AST nodes sequentially:
- Math operations are cleanly evaluated against standard Rust types.
- `PRINT` statements utilize the formatting traits of the `Object` enum to output text directly to standard output.
- `IF`, `FOR`, and `REPEAT WHEN` statements evaluate their inner conditional expressions dynamically to control loop execution blocks.
