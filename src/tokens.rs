// We derive these traits so we can print tokens for debugging (Debug),
// compare them (PartialEq), and copy them easily (Clone).
// Rust gon get mad if i dont camelcase the enums so we do it like this.
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // --- Atomic Keywords ---
    // Every Single Keyword in LEXOR is its own token.
    Declare,
    Print,
    Scan,
    If,
    Else,
    For,
    Repeat,
    When,
    Start,
    End,
    Script,
    Area,

    // --- Data Types ---
    // These match the 4 specific types listed in the spec.
    TypeInt,
    TypeChar,
    TypeBool,
    TypeFloat,

    // --- Literals (The actual data) ---
    Identifier(String), // Variable names like "xyz"
    IntLit(i32),        // Spec says 4 bytes, so we use i32 (not i64 its 8 bytes)
    FloatLit(f32),      // Spec says 4 bytes, so we use f32 (not f64 its 8 bytes)
    BoolLit(bool),      // TRUE or FALSE
    CharLit(char),      // Single symbol
    StringLit(String),  // Needed for PRINT commands like "last"

    // --- Operators ---
    // Basic Arithmetic Operators: + - * / %
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    // Comparison Operators: == <> < > <= >=
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    // Logical Operators: AND OR NOT
    And,
    Or,
    Not,
    // & Concatenation Operator
    Concat,
    // = Assignment Operator
    Assign,

    // --- Structure Symbols ---
    Dollar, // $ (Acts as the statement terminator)
    Comma,  // ,
    LParen, // (
    RParen, // )

    // --- Error Handling ---
    Illegal(String), // Captures any junk characters we don't recognize
}
