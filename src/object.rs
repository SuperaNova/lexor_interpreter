//! The LEXOR Runtime Object System.
//!
//! This module defines the universal `Object` enum wrapper used to safely classify 
//! dynamic runtime values (like Ints, Floats, and Booleans) securely during active evaluation.

use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i32),
    Float(f32),
    Boolean(bool),
    Character(char),
    String(String),
    Null,
    Error(String),
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
            Object::Error(message) => write!(f, "ERROR: {}", message),
        }
    }
}
