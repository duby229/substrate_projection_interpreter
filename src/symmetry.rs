//! Symmetry, attractor, and differentiation detection for SPTL agents.

use crate::agents::Agent;

/// Returns true if all symbols' interpretant histories have stabilized (ΔΠ(s, τ) = 0 for last N steps).
pub fn detect_symmetry(agent: &Agent, window: usize) -> bool {
    for trace in &agent.memory.traces {
        let meanings = &trace.interpretants;
        if meanings.len() < window + 1 {
            return false;
        }
        let last = &meanings[meanings.len() - window..];
        let first_desc = &last[0].description;
        if !last.iter().all(|m| &m.description == first_desc) {
            return false;
        }
    }
    true
}

/// Returns true if any symbol shows recent differentiation (non-stable meaning).
pub fn detect_differentiation(agent: &Agent, window: usize) -> bool {
    for trace in &agent.memory.traces {
        let meanings = &trace.interpretants;
        if meanings.len() < window + 1 {
            continue;
        }
        let last = &meanings[meanings.len() - window..];
        let mut iter = last.iter();
        let mut prev = iter.next().unwrap();
        for m in iter {
            if m.description != prev.description {
                return true;
            }
            prev = m;
        }
    }
    false
}

/// Returns true if all memory traces have stabilized their interpretants (symmetry/attractor).
pub fn detect_attractor(agent: &Agent, window: usize) -> bool {
    detect_symmetry(agent, window)
}