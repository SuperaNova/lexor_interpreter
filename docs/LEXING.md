# LEXOR Lexical Analysis (Scanner)

The foundation of the interpreter begins in `core/src/lexer.rs`. Before the Parser can understand the logic or structure of the code, we must convert the raw text (characters) into categorized, manageable pieces called **Tokens**. This process is known as Lexical Analysis or Scanning.

## The Vocabulary (`core/src/tokens.rs`)
The `Token` enum in `core/src/tokens.rs` acts as the complete dictionary for the LEXOR language. 
- **Keywords:** `DECLARE`, `PRINT`, `IF`, `FOR`, `START`, `END`
- **Types:** `INT`, `FLOAT`, `BOOL`, `CHAR`
- **Operators:** `+`, `-`, `*`, `==`, `&`, `=`
- **Literals:** `IntLit(42)`, `Identifier("area")`
- **Symbols:** `Comma`, `Colon`, `Newline`, `Dollar`

## The Lexer Engine (`core/src/lexer.rs`)
The `Lexer` struct wraps the source code and iterates over a `Peekable` stream of characters.

### Core Mechanics
1. **Skipping Whitespace:** The lexer automatically ignores spaces (` `) and tabs (`\t`) to keep the token stream clean. However, it explicitly **preserves** Newlines (`\n`) returning them as `Token::Newline` because the LEXOR grammar often structurally relies on them.
2. **Lookahead Matching:** The lexer uses a `peek_char()` function to look one character ahead without actually consuming it. This is critical for distinguishing between `<` (Less Than) and `<=` (Less Than or Equal), or single `=` (Assignment) versus double `==` (Equality).
3. **Escaped Characters:** A dedicated `read_escape_sequence()` function flawlessly handles LEXOR's unique `[#]`, `[[]`, and `[]]` syntax by collecting the interior characters and safely translating them to the appropriate character literal.
4. **Rust Iterator Integration:** The `Lexer` implements Rust's standard generic `Iterator` trait. This heavily modernizes the interpreter, meaning the Parser later can natively stream tokens safely, reducing tracking logic and bugs.

### Extending the Language
If you ever want to add a completely new structural word to the language (for example, a `WHILE` loop):
1. Add a `While` definition to the `Token` enum in `core/src/tokens.rs`.
2. Add `"WHILE" => Token::While` into the `match ident.as_str()` switch block inside the `read_identifier` function in `core/src/lexer.rs`.
And the Lexer will automatically begin tagging those words for the Parser!
