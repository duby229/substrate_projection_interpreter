/*
 * This file is part of SPTL-SPI.
 *
 * SPTL-SPI is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * SPTL-SPI is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with SPTL-SPI.  If not, see <https://www.gnu.org/licenses/>.
 */

//! Substrate (●) and pattern type for SPTL.
//!
//! # Harmonized SPTL Principle
//!
//! - **Substrate (●):** A field of activations that are always decaying, always available for projection and resonance. If it can be activated and decayed, it is substrate.
//!

use std::collections::HashMap;
use rayon::prelude::*; // For parallelism
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
    /// Parallelized with Rayon.
    pub fn decay(&mut self, rate: f64) {
        self.activations.par_iter_mut().for_each(|(_pat, v)| {
            *v = (*v * (1.0 - rate)).max(0.0);
        });
        self.activations.retain(|_, v| *v > 0.01);
    }
}