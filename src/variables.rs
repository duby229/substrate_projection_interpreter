//! Symbolic variable support for SPTL shell.

use std::collections::HashMap;

#[derive(Clone)]
pub enum SymbolicValue {
    Symbol { token: String, pattern: String },
    Pattern(String),
    Token(String),
    // Extend as needed
}

#[derive(Default)]
pub struct VariableTable {
    table: HashMap<String, SymbolicValue>,
}

impl VariableTable {
    pub fn new() -> Self {
        Self { table: HashMap::new() }
    }

    pub fn set(&mut self, name: &str, value: SymbolicValue) {
        self.table.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&SymbolicValue> {
        self.table.get(name)
    }
}