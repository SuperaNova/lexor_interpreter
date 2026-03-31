use std::collections::HashMap;
use crate::object::Object;

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
