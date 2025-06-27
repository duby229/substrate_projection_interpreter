//! Symbol/sign (※) and meaning module for SPTL.
//!
//! # Harmonized SPTL Principle
//!
//! - **Sign (※):** Not statically defined, but whatever an agent expresses and stabilizes through τ-indexed cycles. A sign is validated by participating in the say → project → interpret loop and persisting through tick.
//! - **Meaning:** Created by interpretation at a specific τ. Meaning is always situated in process and recursively re-enacted.
//!
//! See SPTL-Specification-Harmonization.md for more.

use crate::substrate::Pattern;

/// A symbolic sign: a token and a pattern.
/// Signs are not static; their identity emerges from cycles of expression, projection, and interpretation.
/// If it participates in the say → project → interpret loop and survives tick, it is a sign.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    /// The sign's token (e.g. word, name, identifier).
    pub token: String,
    /// The associated pattern.
    pub pattern: Pattern,
}

impl Symbol {
    /// Construct a new symbol (token, pattern pair).
    pub fn new(token: &str, pattern: Pattern) -> Self {
        Symbol {
            token: token.to_string(),
            pattern,
        }
    }

    /// Return a mutated version of the symbol (e.g. for drift/inheritance).
    pub fn mutate(&self) -> Symbol {
        let mutated = format!("{}*", self.token);
        Symbol::new(&mutated, self.pattern.clone())
    }
}

/// A meaning is an interpretation of a symbol at a recursion index (tau).
/// Meaning is always situated in τ; it only exists as an interpretive event.
#[derive(Debug, Clone)]
pub struct Meaning {
    /// The sign/symbol being interpreted.
    pub sign: Symbol,
    /// The recursion/time index at which this meaning was created.
    pub tau: usize,
    /// Human-readable description of the meaning.
    pub description: String,
}

impl Meaning {
    /// Create a new meaning from a symbol and recursion index.
    pub fn from_symbol(symbol: &Symbol, tau: usize) -> Self {
        Meaning {
            sign: symbol.clone(),
            tau,
            description: format!("Interpretation of '{}' at τ={}", symbol.token, tau),
        }
    }
}