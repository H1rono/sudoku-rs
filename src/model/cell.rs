use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
pub enum Cell {
    Empty { candidates: HashSet<u32> },
    Number(u32),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty { .. })
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Cell::Number(_))
    }

    pub fn number(&self) -> Option<u32> {
        match self {
            Cell::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn candidates(&self) -> Option<&HashSet<u32>> {
        match self {
            Cell::Empty { candidates } => Some(candidates),
            _ => None,
        }
    }

    pub fn remove_candidate(&mut self, candidate: u32) {
        if let Cell::Empty { candidates } = self {
            candidates.remove(&candidate);
        }
    }

    pub fn set_number(&mut self, number: u32) {
        *self = Cell::Number(number);
    }

    pub fn set_candidates(&mut self, candidates: HashSet<u32>) {
        *self = Cell::Empty { candidates };
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty { candidates } => {
                let s = candidates.iter().map(|n| n.to_string()).join(",");
                write!(f, "[{}]", s)
            }
            Cell::Number(n) => write!(f, "{}", n),
        }
    }
}
