//! The LEXOR Runtime Environment Memory Cache.
//!
//! This module provides the `Environment` struct, acting as the immediate RAM during execution.
//! It wraps a HashMap to natively store and retrieve dynamically declared variables and their strict types reliably.

use crate::object::Object;
use crate::tokens::Token;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, (Token, Object)>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    /// Queries the memory store for a saved variable.
    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name).map(|(_, obj)| obj)
    }

    /// Queries the memory store for a saved variable's explicit type.
    pub fn get_type(&self, name: &str) -> Option<&Token> {
        self.store.get(name).map(|(t, _)| t)
    }

    /// Inserts or updates a uniquely typed value safely cleanly natively in the memory store.
    pub fn set(&mut self, name: String, var_type: Token, val: Object) {
        self.store.insert(name, (var_type, val));
    }
}
