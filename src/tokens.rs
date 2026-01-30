/*
 * * The Token System (Vocabulary)
 * * This file defines the "Dictionary" of the LEXOR language.
 * * It lists every possible word, symbol, and value that the interpreter understands.
 * * * Its main jobs are:
 * 1. Define Atomic Keywords (e.g., DECLARE, PRINT, IF) so the Lexer can tag them.
 * 2. Define Data Types strictly according to the spec:
 * - INT   -> i32 (4 bytes)
 * - FLOAT -> f32 (4 bytes)
 * - CHAR  -> char
 * - BOOL  -> bool
 * 3. Define Operators for Math (+, -) and Logic (AND, OR, NOT).
 * 4. Serve as the shared "language" that lets the Lexer talk to the Parser.
 */

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
    Newline, // Represents the actual end of a line of code (\n)
    Dollar,  // $ (Used in PRINT for outputting a new line)
    Comma,   // ,
    LParen,  // (
    RParen,  // )

    // --- Error Handling ---
    Illegal(String), // Captures any junk characters we don't recognize
}
