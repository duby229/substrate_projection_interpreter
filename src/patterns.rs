//! Pattern abstraction support for SPTL shell.

use std::collections::HashMap;

/// Manages named patterns for symbolic reuse.
#[derive(Default)]
pub struct PatternTable {
    table: HashMap<String, String>,
}

impl PatternTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    /// Define or update a named pattern.
    pub fn define(&mut self, name: &str, value: &str) {
        self.table.insert(name.to_string(), value.to_string());
    }

    /// Expand pattern references in the input string.
    /// Replaces any [name] with its pattern definition.
    pub fn expand_patterns<'a>(&'a self, text: &'a str) -> String {
        let mut out = String::new();
        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '[' {
                let mut name = String::new();
                while let Some(&n) = chars.peek() {
                    if n == ']' {
                        chars.next();
                        break;
                    }
                    name.push(n);
                    chars.next();
                }
                if let Some(val) = self.table.get(&name) {
                    out.push_str(val);
                } else {
                    out.push('[');
                    out.push_str(&name);
                    out.push(']');
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}