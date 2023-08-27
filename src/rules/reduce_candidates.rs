use std::{collections::HashSet, ops::Not, rc::Rc};

use thiserror::Error as ThisError;

use super::Rule;

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("no candidate in cell ({row}, {col})")]
    NoCandidate { row: u32, col: u32 },
}

pub struct ReduceCandidates;

impl Rule for ReduceCandidates {
    type Error = Error;

    fn apply(&self, board: &mut crate::Board) -> Result<(), Self::Error> {
        let sq_base = board.sq_base();
        let it = (0..sq_base)
            .flat_map(|row| (0..sq_base).map(move |col| (row, col)))
            .map(|(row, col)| (row, col, board.get(row, col).unwrap()))
            .filter(|&(_, _, cell)| cell.is_empty())
            .map(|(row, col, cell)| {
                let candidates = cell.candidates().unwrap();
                let mut numbers = board.row(row).numbers();
                numbers.extend(board.col(col).numbers());
                numbers.extend(board.square(row, col).numbers());
                let d: HashSet<_> = candidates.difference(&numbers).cloned().collect();
                d.is_empty()
                    .not()
                    .then_some((row, col, d))
                    .ok_or(Error::NoCandidate { row, col })
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (row, col, d) in it {
            let cell = Rc::get_mut(board.get_mut(row, col).unwrap()).unwrap();
            cell.set_candidates(d);
        }
        Ok(())
    }
}
