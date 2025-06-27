//! Agent (⟁) — See SPT Section IV, VII.
//! Identity enacted through recursive sign cycles.

use std::collections::{HashMap, VecDeque};
use crate::substrate::{Substrate, Pattern};
use crate::symbol::{Symbol, Meaning};
// ... other use statements unchanged

// ... MemoryTrace, MemoryField unchanged

#[derive(Debug)]
pub struct Agent {
    /// Agent identifier.
    pub id: String,
    /// All known symbols (token → pattern).
    pub symbol_table: HashMap<String, Pattern>,
    /// Agent's memory field.
    pub memory: MemoryField,
    /// Minimum stability required for memory admission.
    pub coherence_threshold: f64,
}

impl Agent {
    // ... existing methods unchanged

    /// Returns true if all memory traces have stabilized their interpretants (symmetry/attractor).
    /// See SPT Section VII.
    pub fn is_attractor_state(&self, window: usize) -> bool {
        crate::symmetry::detect_attractor(self, window)
    }

    /// Parallelized tick for this agent (decay, reinforce, etc.)
    pub fn tick_parallel(&mut self) {
        self.decay_memory(0.05);
        // You may add more parallelized behavior here as needed.
    }
}

// Make Agent Send + Sync for Rayon/threads
unsafe impl Send for Agent {}
unsafe impl Sync for Agent {}