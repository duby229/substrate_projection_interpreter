//! Agents (⟁), memory field (◐), and recursive interpretation (Πₙ) module.
//! 
//! # Harmonized SPTL Principle
//! 
//! - **Agent (⟁):** Not merely a static data structure, but a *recursive semiotic attractor*—an entity whose identity and persistence are produced by its ongoing cycles of sign expression, projection, interpretation, and memory decay. If it recursively stabilizes and mutates signs, it is an agent.
//! - **Memory (◐):** Not a fixed store, but a dynamic field of traces always at risk of decay, always subject to reinforcement. Memory is whatever persists through feedback and interpretive cycles.
//! - **Interpretation (Π):** The act of making meaning, situated within the τ-indexed context; reinforces and potentially mutates the memory field.
//! 
//! See SPTL-Specification-Harmonization.md for more on behavioral ontology.

use std::collections::{HashMap, VecDeque};
use crate::substrate::{Substrate, Pattern};
use crate::symbol::{Symbol, Meaning};

/// Represents a memory trace for a symbol, with stability and interpretants.
/// A memory trace is not static: its stability emerges through feedback cycles.
#[derive(Debug, Clone)]
pub struct MemoryTrace {
    /// The symbol this trace refers to.
    pub symbol: Symbol,
    /// The recursion/time index when it was created.
    pub tau_index: usize,
    /// Stability [0,1] of the trace.
    pub stability: f64,
    /// All meanings/interpretations of the symbol for this trace.
    pub interpretants: Vec<Meaning>,
}

impl MemoryTrace {
    /// Reinforces the trace, increasing stability.
    pub fn reinforce(&mut self, delta: f64) {
        self.stability = (self.stability + delta).clamp(0.0, 1.0);
    }
    /// Decays the trace, decreasing stability.
    pub fn decay(&mut self, rate: f64) {
        self.stability = (self.stability - rate).max(0.0);
    }
}

/// MemoryField stores a queue of memory traces for an agent.
/// Memory is always dynamic, subject to decay and feedback.
#[derive(Debug, Default)]
pub struct MemoryField {
    /// The traces currently stored.
    pub traces: VecDeque<MemoryTrace>,
    /// Maximum number of traces to store.
    pub max_traces: usize,
}

impl MemoryField {
    /// Admit a new trace if stability ≥ eta, evicting oldest if at capacity.
    pub fn admit(&mut self, trace: MemoryTrace, eta: f64) {
        if trace.stability >= eta {
            if self.traces.len() >= self.max_traces {
                self.traces.pop_front();
            }
            self.traces.push_back(trace);
        }
    }
    /// Reinforce stability for a matching symbol.
    pub fn reinforce_symbol(&mut self, symbol: &Symbol, delta: f64) {
        for t in &mut self.traces {
            if &t.symbol == symbol {
                t.reinforce(delta);
            }
        }
    }
    /// Decay all traces, removing those below threshold.
    pub fn decay_all(&mut self, rate: f64) {
        for t in &mut self.traces {
            t.decay(rate);
        }
        self.traces.retain(|t| t.stability > 0.0);
    }
    /// Find a trace by symbol.
    pub fn find(&self, symbol: &Symbol) -> Option<&MemoryTrace> {
        self.traces.iter().find(|t| &t.symbol == symbol)
    }
}

/// Symbolic agent (⟁): owns memory, a sign table, and core parameters.
/// Agents are recursive processes: their identity is enacted through cycles of sign expression, projection, and interpretation.
/// See SPTL-Specification-Harmonization.md for principle: "If it recursively stabilizes and mutates signs, it is an agent."
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
    /// Construct a new agent with given memory and coherence.
    pub fn new(id: impl Into<String>, max_memory: usize, coherence_threshold: f64) -> Self {
        Agent {
            id: id.into(),
            symbol_table: HashMap::new(),
            memory: MemoryField {
                traces: VecDeque::with_capacity(max_memory),
                max_traces: max_memory,
            },
            coherence_threshold,
        }
    }

    /// Express a symbol (token, pattern), adding a trace if stable.
    /// In SPTL, expression is an act that can recursively reinforce or mutate the system.
    pub fn express_symbol(&mut self, token: &str, pattern: Pattern, tau: usize) -> Symbol {
        let symbol = Symbol::new(token, pattern.clone());
        self.symbol_table.insert(token.to_string(), pattern);
        let trace = MemoryTrace {
            symbol: symbol.clone(),
            tau_index: tau,
            stability: 1.0,
            interpretants: Vec::new(),
        };
        self.memory.admit(trace, self.coherence_threshold);
        symbol
    }

    /// Project a symbol into the substrate.
    pub fn project_symbol(&self, symbol: &Symbol, substrate: &mut Substrate) {
        substrate.project(symbol);
    }

    /// Attempt to interpret a symbol, reinforcing memory if successful.
    pub fn interpret_symbol(&mut self, symbol: &Symbol, tau: usize) -> Option<Meaning> {
        if let Some(pattern) = self.symbol_table.get(&symbol.token) {
            if pattern == &symbol.pattern {
                self.memory.reinforce_symbol(symbol, 0.1);
                let meaning = Meaning::from_symbol(symbol, tau);
                if let Some(trace) = self.memory.traces.iter_mut().find(|t| &t.symbol == symbol) {
                    trace.interpretants.push(meaning.clone());
                }
                return Some(meaning);
            }
        }
        None
    }

    /// Return a mutated version of the symbol.
    pub fn mutate_symbol(&self, symbol: &Symbol) -> Symbol {
        symbol.mutate()
    }

    /// Decay all memory traces.
    pub fn decay_memory(&mut self, rate: f64) {
        self.memory.decay_all(rate);
    }
}