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

use crate::recursion::{CategoryObject, RecursionLevel};
use crate::interpretation::Interpretation;

use std::collections::HashMap;

pub struct Shell {
    pub categories: HashMap<String, CategoryObject>,
    // ... other fields ...
}

impl Shell {
    // ... other methods ...

    /// Show interpretation at any level by id.
    pub fn handle_interpret(&self, args: &[String]) {
        if args.len() < 2 {
            println!("Usage: interpret <level> <id>");
            return;
        }
        let level = &args[0];
        let id = &args[1];
        if let Some(obj) = self.categories.get(id) {
            match obj.interpret() {
                Some(interpretation) => {
                    println!("Interpretation at level {:?} for {}:\n{:#?}", obj.level, id, interpretation);
                }
                None => {
                    println!("No interpretation available for {} at level {:?}", id, obj.level);
                }
            }
        } else {
            println!("Category object '{}' not found.", id);
        }
    }
}