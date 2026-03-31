//! The LEXOR Token Dictionary.
//!
//! This module defines the complete, finite vocabulary ("Dictionary") for the LEXOR programming language.
//! It serves as the exclusive shared explicit language that allows the Lexer to safely communicate with the Parser.
//!
//! # Core Responsibilities
//! 1. **Atomic Keywords:** Defines strict structural bounds explicitly (`DECLARE`, `PRINT`, `IF`).
//! 2. **Data Types:** Maps LEXOR specifications safely to explicit native Rust equivalents:
//!    - `INT` natively safely maps to a 4-byte `i32`.
//!    - `FLOAT` resolves securely explicitly to `f32`.
//!    - `CHAR` and `BOOL` specifically translate structurally to `char` and `bool` flags.
//! 3. **Operator Bindings:** Safely uniformly catalogs native mathematics (`+`, `-`) and strict abstract logic gates (`AND`, `OR`, `NOT`).

/// Enums are structurally explicitly defined securely in PascalCase to satisfy explicitly strict idiomatic Rust compilation.
/// We explicitly derive Debug (for logging natively), PartialEq (for safe explicit equality checks), and Clone (to safely explicitly dynamically replicate tokens identically out to the Parser).
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
    Newline, // Represents the actual end of a line of code (\n)
    Dollar,  // $ (Used in PRINT for outputting a new line)
    Comma,   // ,
    Colon,   // : (Required after PRINT and SCAN)
    LParen,  // (
    RParen,  // )

    // --- Error Handling ---
    Illegal(String), // Captures any junk characters we don't recognize
}
