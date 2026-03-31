//! The LEXOR Runtime Environment Memory Cache.
//!
//! This module provides the `Environment` struct, acting as the immediate RAM during execution.
//! It wraps a HashMap to natively store and retrieve dynamically declared variables securely.

use crate::object::Object;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    /// Queries the memory store for a saved variable.
    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    /// Inserts or updates a value in the memory store.
    pub fn set(&mut self, name: String, val: Object) {
        self.store.insert(name, val);
    }
}
