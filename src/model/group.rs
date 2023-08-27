use std::{collections::HashSet, fmt::Display, rc::Rc};

use super::Cell;

#[derive(Debug)]
pub struct Group {
    cells: Vec<Rc<Cell>>,
}

impl Group {
    pub fn new(cells: Vec<Rc<Cell>>) -> Self {
        Self { cells }
    }

    pub fn is_valid(&self) -> bool {
        let mut numbers = HashSet::new();
        for cell in &self.cells {
            if let Some(number) = cell.number() {
                if numbers.contains(&number) {
                    return false;
                }
                numbers.insert(number);
            }
        }
        true
    }

    pub fn is_solved(&self) -> bool {
        for cell in &self.cells {
            if cell.is_empty() {
                return false;
            }
        }
        true
    }

    pub fn remove_candidate(&mut self, candidate: u32) {
        for cell in &mut self.cells {
            Rc::get_mut(cell).unwrap().remove_candidate(candidate);
        }
    }

    pub fn numbers(&self) -> HashSet<u32> {
        self.cells.iter().filter_map(|cell| cell.number()).collect()
    }

    pub fn candidates(&self) -> HashSet<u32> {
        self.cells
            .iter()
            .filter_map(|cell| cell.candidates().cloned())
            .flatten()
            .collect()
    }
}

impl Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = self.cells.len();
        for (i, cell) in self.cells.iter().enumerate() {
            write!(f, "{}", cell)?;
            if i < l {
                writeln!(f, " ")?;
            }
        }
        Ok(())
    }
}
