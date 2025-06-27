//! Macro support for SPTL shell.

use std::collections::HashMap;

pub struct Macro {
    pub params: Vec<String>,
    pub body: String,
}

#[derive(Default)]
pub struct MacroTable {
    table: HashMap<String, Macro>,
}

impl MacroTable {
    pub fn new() -> Self {
        Self { table: HashMap::new() }
    }

    pub fn define(&mut self, name: &str, params: Vec<String>, body: String) {
        self.table.insert(name.to_string(), Macro { params, body });
    }

    pub fn get(&self, name: &str) -> Option<&Macro> {
        self.table.get(name)
    }
}