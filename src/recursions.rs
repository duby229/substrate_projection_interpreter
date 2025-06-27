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

//! Recursion category stack up to Λ₄ (cells), with cross-level feedback, interpretation, and upward/downward causation support.

use crate::agents::Agent;
use crate::substrate::Substrate;
use crate::interpretation::*;
use std::collections::HashMap;
use rayon::prelude::*;

/// Enum for the recursion/categorical level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecursionLevel {
    Void,       // Λ₀
    Particle,   // Λ₁
    Atom,       // Λ₂
    Molecule,   // Λ₃
    Cell,       // Λ₄
}

#[derive(Debug)]
pub struct CategoryObject {
    pub level: RecursionLevel,
    pub id: String,
    pub substrate: Substrate,
    pub subobjects: Vec<Box<CategoryObject>>,
    pub agents: Vec<Agent>,
}

impl CategoryObject {
    pub fn new(level: RecursionLevel, id: &str) -> Self {
        Self {
            level,
            id: id.to_string(),
            substrate: Substrate::default(),
            subobjects: Vec::new(),
            agents: Vec::new(),
        }
    }

    /// "Promote" this object to the next recursion level, wrapping as a subobject
    pub fn promote(self) -> Option<CategoryObject> {
        use RecursionLevel::*;
        let next_level = match self.level {
            Void => Particle,
            Particle => Atom,
            Atom => Molecule,
            Molecule => Cell,
            Cell => return None,
        };
        Some(CategoryObject {
            level: next_level,
            id: format!("{}-{}", next_level as u8, self.id),
            substrate: Substrate::default(),
            subobjects: vec![Box::new(self)],
            agents: Vec::new(),
        })
    }

    /// Recursively tick all subobjects and agents in parallel.
    pub fn tick_recursive(&mut self) {
        self.subobjects.par_iter_mut().for_each(|sub| sub.tick_recursive());
        self.agents.par_iter_mut().for_each(|agent| agent.decay_memory(0.05));
        self.substrate.decay(0.05);
    }

    /// Recursively propagate a mutation (cross-level feedback) down to all subobjects and agents.
    pub fn propagate_mutation(&mut self, message: &str) {
        self.agents.par_iter_mut().for_each(|agent| {
            agent.express_symbol(&format!("mutation:{}", message), crate::substrate::Pattern::new("111"), 0);
        });
        self.subobjects.par_iter_mut().for_each(|sub| sub.propagate_mutation(message));
    }

    /// Recursively aggregate a value upward (example of upward causation).
    pub fn aggregate_stability(&self) -> f64 {
        let sub_sum: f64 = self.subobjects.par_iter().map(|sub| sub.aggregate_stability()).sum();
        let agent_sum: f64 = self.agents.par_iter().map(|agent| {
            agent.memory.traces.iter().map(|t| t.stability).sum::<f64>()
        }).sum();
        sub_sum + agent_sum
    }

    /// --- INTERPRETATION METHODS FOR ALL LEVELS ---

    /// Unified interpretation entrypoint.
    pub fn interpret(&self) -> Option<Interpretation> {
        match self.level {
            RecursionLevel::Particle => Some(Interpretation::Particle(self.interpret_particle())),
            RecursionLevel::Atom => Some(Interpretation::Atom(self.interpret_atom())),
            RecursionLevel::Molecule => Some(Interpretation::Molecule(self.interpret_molecule())),
            RecursionLevel::Cell => Some(Interpretation::Cell(self.interpret_cell())),
            RecursionLevel::Void => None,
        }
    }

    /// Λ₁: Particle-level interpretation.
    pub fn interpret_particle(&self) -> ParticleInterpretation {
        ParticleInterpretation {
            id: self.id.clone(),
            quantum_state: format!("q{:x}", self.substrate.activations.len()), // toy example
            energy: self.substrate.activations.values().sum(),
        }
    }

    /// Λ₂: Atom-level interpretation.
    pub fn interpret_atom(&self) -> AtomInterpretation {
        let constituent_particles: Vec<ParticleInterpretation> = self.subobjects.iter()
            .filter(|s| s.level == RecursionLevel::Particle)
            .filter_map(|s| s.interpret().and_then(|i| match i { Interpretation::Particle(p) => Some(p), _ => None }))
            .collect();
        AtomInterpretation {
            id: self.id.clone(),
            atomic_number: constituent_particles.len() as u32, // toy logic
            shell_config: format!("{}s2", constituent_particles.len()), // toy
            constituent_particles,
        }
    }

    /// Λ₃: Molecule-level interpretation.
    pub fn interpret_molecule(&self) -> MoleculeInterpretation {
        let constituent_atoms: Vec<AtomInterpretation> = self.subobjects.iter()
            .filter(|s| s.level == RecursionLevel::Atom)
            .filter_map(|s| s.interpret().and_then(|i| match i { Interpretation::Atom(a) => Some(a), _ => None }))
            .collect();
        let bonds = if constituent_atoms.len() > 1 {
            (0..constituent_atoms.len()-1).map(|i| {
                format!("{}-{}", constituent_atoms[i].id, constituent_atoms[i+1].id)
            }).collect()
        } else {
            Vec::new()
        };
        MoleculeInterpretation {
            id: self.id.clone(),
            formula: format!("Molecule{}{}", self.id, constituent_atoms.len()),
            bonds,
            constituent_atoms,
        }
    }

    /// Λ₄: Cell-level interpretation (emergent/holistic).
    pub fn interpret_cell(&self) -> CellInterpretation {
        let contributing_meanings: Vec<String> = self.subobjects.iter()
            .flat_map(|sub| sub.interpret())
            .map(|interp| format!("{:?}", interp))
            .collect();
        CellInterpretation {
            id: self.id.clone(),
            summary: format!("Cell {} integrates {} sub-meanings", self.id, contributing_meanings.len()),
            emergent_properties: vec!["homeostasis".to_string()],
            contributing_meanings,
        }
    }
}