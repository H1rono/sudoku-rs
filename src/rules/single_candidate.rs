use std::{ops::Not, rc::Rc};

use thiserror::Error as ThisError;

use super::Rule;

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("invalid candidate {candidate} in cell ({row}, {col})")]
    InvalidCandidate { row: u32, col: u32, candidate: u32 },
}

pub struct SingleCandidate;

impl Rule for SingleCandidate {
    type Error = Error;

    fn apply(&self, board: &mut crate::Board) -> Result<(), Self::Error> {
        let sq_base = board.sq_base();
        let it = (0..sq_base)
            .flat_map(|row| (0..sq_base).map(move |col| (row, col)))
            .map(|(row, col)| (row, col, board.get(row, col).unwrap()))
            .filter(|&(_, _, cell)| cell.is_empty())
            .map(|(row, col, cell)| (row, col, cell.candidates().unwrap()))
            .filter(|&(_, _, candidates)| candidates.len() == 1)
            .map(|(row, col, candidates)| {
                let candidate = *candidates.iter().next().unwrap();
                let mut numbers = board.row(row).numbers();
                numbers.extend(board.col(col).numbers());
                numbers.extend(board.square(row, col).numbers());
                numbers
                    .contains(&candidate)
                    .not()
                    .then_some((row, col, candidate))
                    .ok_or(Error::InvalidCandidate {
                        row,
                        col,
                        candidate,
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        for (row, col, n) in it {
            let cell = Rc::get_mut(board.get_mut(row, col).unwrap()).unwrap();
            cell.set_number(n);
        }
        Ok(())
    }
}
