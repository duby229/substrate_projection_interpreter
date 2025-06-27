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
 
//! Structured interpretations for all recursion levels (Λ₁, Λ₂, Λ₃, Λ₄) in SPTL.

#[derive(Debug, Clone)]
pub enum Interpretation {
    Particle(ParticleInterpretation), // Λ₁
    Atom(AtomInterpretation),         // Λ₂
    Molecule(MoleculeInterpretation), // Λ₃
    Cell(CellInterpretation),         // Λ₄
}

/// Λ₁: Particle-level interpretation (e.g., quantum state)
#[derive(Debug, Clone)]
pub struct ParticleInterpretation {
    pub id: String,
    pub quantum_state: String,
    pub energy: f64,
}

/// Λ₂: Atom-level interpretation (e.g., atomic number, orbitals)
#[derive(Debug, Clone)]
pub struct AtomInterpretation {
    pub id: String,
    pub atomic_number: u32,
    pub shell_config: String,
    pub constituent_particles: Vec<ParticleInterpretation>,
}

/// Λ₃: Molecule-level interpretation (e.g., formula, bonds)
#[derive(Debug, Clone)]
pub struct MoleculeInterpretation {
    pub id: String,
    pub formula: String,
    pub bonds: Vec<String>,
    pub constituent_atoms: Vec<AtomInterpretation>,
}

/// Λ₄: Cell-level interpretation (emergent/holistic)
#[derive(Debug, Clone)]
pub struct CellInterpretation {
    pub id: String,
    pub summary: String,
    pub emergent_properties: Vec<String>,
    pub contributing_meanings: Vec<String>,
}