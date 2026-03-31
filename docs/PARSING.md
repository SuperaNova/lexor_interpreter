# LEXOR Parsing Engines

Because parsing raw text into executable logic is complex, `src/parser.rs` uses two distinct industry-standard parsing algorithms layered together:

## Top-Down Recursive Descent (Statement Parsing)
Used heavily for parsing structural blocks like `IF`, `FOR`, `REPEAT WHEN`, and `DECLARE` statements.
- **How it works:** `parse_program` starts at the very top and reads tokens. When it encounters `DECLARE`, it jumps to `parse_declare_statement()`, which tightly expects specific tokens (like identifiers or types). 
- **Validation:** This architecture allows us to fail gracefully in exact places, outputting user-friendly errors like "Expected ':' after PRINT".

## Pratt Parsing (Expression Parsing)
Used exclusively to parse expressions (like `5 * 10 + 2` or `x = y = 4`) and mathematically maintain strict **Order of Operations** (Precedence) without horrific spaghetti code.
- **The Problem:** Recursive Descent is terrible at mathematics. It requires dozens of deeply-nested functions to ensure multiplication happens before addition.
- **The Pratt Solution:** `parse_expression` relies on a central `Precedence Table` built into `parser.rs`. Each mathematical operator (`*`, `+`, `=`, `<`, etc.) is assigned a weight (`PREC_PRODUCT = 8`, `PREC_SUM = 7`).
- **In Action:** While parsing, if an operator arrives with a higher precedence weight than the current level, it binds the syntax node tighter to itself, ensuring `5 + (10 * 2)` is structured cleanly without nested boilerplate methods.
- **Right associativity:** Operators like `=` subtract `1` from their precedence lookup natively, allowing an `x = y = z` structure to naturally bind strictly to the right exactly like modern languages.

### Extending the Language
If you ever need to add a newly invented Operator to the LEXOR language:
1. Add the token symbol to `Lexer.rs`.
2. Add the operator to the `get_precedence` lookup table inside `parser.rs`.
3. Add it to `is_infix` inside the parser.
And the Pratt parsing engine will automatically begin building perfect mathematically-sorted ASTs for it without you manually writing a single line of logical checks!
