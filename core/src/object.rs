//! The LEXOR Runtime Object System.
//!
//! This module defines the universal `Object` enum wrapper used to safely classify
//! dynamic runtime values (like Ints, Floats, and Booleans) securely during active evaluation.
//!
//! # Error Handling
//! Runtime errors are represented as [`LexorError`] variants rather than bare strings,
//! so callers can pattern-match on the *category* of the failure.

use std::fmt;

/// Categorised runtime errors produced by the LEXOR evaluator.
#[derive(Debug, PartialEq, Clone)]
pub enum LexorError {
    /// A value of the wrong type was supplied to a DECLARE, SCAN, or assignment.
    TypeError {
        /// Human-readable name of the expected type (e.g. `"BOOL"`).
        expected: String,
        /// Display string of the value that was actually received.
        got: String,
        /// Where the mismatch occurred (e.g. `"SCAN input for 't1'"`).
        context: String,
    },

    /// An identifier was used before it was `DECLARE`d.
    UndeclaredVariable { name: String },

    /// Integer or float division / modulo by zero.
    DivisionByZero { detail: String },

    /// An operator was applied to an operand of an unsupported type.
    InvalidOperator { op: String, context: String },

    /// The left-hand side of an assignment was not a valid identifier.
    InvalidAssignmentTarget { detail: String },
}

impl fmt::Display for LexorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexorError::TypeError {
                expected,
                got,
                context,
            } => write!(
                f,
                "[TypeError] {}\n  Expected : {}\n  Got      : {}",
                context, expected, got
            ),
            LexorError::UndeclaredVariable { name } => write!(
                f,
                "[UndeclaredVariable] '{}' was used before being declared with DECLARE",
                name
            ),
            LexorError::DivisionByZero { detail } => {
                write!(f, "[DivisionByZero] {}", detail)
            }
            LexorError::InvalidOperator { op, context } => write!(
                f,
                "[InvalidOperator] Operator '{}' cannot be applied here — {}",
                op, context
            ),
            LexorError::InvalidAssignmentTarget { detail } => {
                write!(f, "[InvalidAssignmentTarget] {}", detail)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Character(char),
    String(String),
    Null,
    /// A structured runtime error. Always causes immediate program termination.
    Error(LexorError),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::Float(value) => write!(f, "{}", value),
            Object::Boolean(value) => {
                // Formatting TRUE/FALSE identically to how the lexer expects them
                if *value {
                    write!(f, "TRUE")
                } else {
                    write!(f, "FALSE")
                }
            }
            Object::Character(value) => write!(f, "{}", value),
            Object::String(value) => write!(f, "{}", value),
            Object::Null => write!(f, "NULL"),
            Object::Error(err) => write!(f, "{}", err),
        }
    }
}
