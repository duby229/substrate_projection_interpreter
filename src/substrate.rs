//! Substrate (●) and pattern type for SPTL.
//!
//! # Harmonized SPTL Principle
//!
//! - **Substrate (●):** A field of activations that are always decaying, always available for projection and resonance. If it can be activated and decayed, it is substrate.
//!

use std::collections::HashMap;
use crate::symbol::Symbol;

/// Represents a symbolic pattern (e.g., a bitstring, glyph, etc).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pattern(pub String);

impl Pattern {
    /// Construct a new pattern from a string.
    pub fn new(s: &str) -> Self {
        Pattern(s.to_string())
    }
}

/// The substrate (●) is a field of activations for patterns.
/// It is always in flux: activations rise upon projection and decay over τ.
#[derive(Debug, Default)]
pub struct Substrate {
    /// Activation level for each pattern present in the substrate.
    pub activations: HashMap<Pattern, f64>,
}

impl Substrate {
    /// Project a symbol into the substrate, increasing its activation.
    pub fn project(&mut self, symbol: &Symbol) {
        let ent = self.activations.entry(symbol.pattern.clone()).or_insert(0.0);
        *ent += 1.0;
    }

    /// Decay all activations multiplicatively, removing those below threshold.
    pub fn decay(&mut self, rate: f64) {
        for v in self.activations.values_mut() {
            *v = (*v * (1.0 - rate)).max(0.0);
        }
        self.activations.retain(|_, v| *v > 0.01);
    }
}