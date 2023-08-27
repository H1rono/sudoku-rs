use std::{collections::HashSet, rc::Rc};

use thiserror::Error as ThisError;

use super::Rule;

#[derive(Debug, Clone, ThisError)]
pub enum Error {
    #[error("multipe candidates {candidates:?} in cell ({row}, {col})")]
    MultipleUniqueCandidates {
        row: u32,
        col: u32,
        candidates: HashSet<u32>,
    },
}

pub struct UniqueCandidate;

impl Rule for UniqueCandidate {
    type Error = Error;

    fn apply(&self, board: &mut crate::Board) -> Result<(), Self::Error> {
        let sq_base = board.sq_base();
        let it = (0..sq_base)
            .flat_map(|row| (0..sq_base).map(move |col| (row, col)))
            .map(|(row, col)| (row, col, board.get(row, col).unwrap()))
            .filter(|&(_, _, cell)| cell.is_empty())
            .map(|(row, col, cell)| {
                let candidates = cell.candidates().unwrap().clone();
                let other_candidates = board.col_except_cell(row, col).candidates();
                let d_col: HashSet<_> = candidates.difference(&other_candidates).cloned().collect();
                let other_candidates = board.row_except_cell(row, col).candidates();
                let d_row: HashSet<_> = candidates.difference(&other_candidates).cloned().collect();
                let other_candidates = board.square_except_cell(row, col).candidates();
                let d_square: HashSet<_> =
                    candidates.difference(&other_candidates).cloned().collect();
                let d = d_col.union(&d_row).cloned().collect::<HashSet<_>>();
                let d = d.union(&d_square).cloned().collect::<HashSet<_>>();
                (row, col, d)
            })
            .collect::<Vec<_>>();
        for (row, col, d) in it {
            use std::cmp::Ordering;
            match d.len().cmp(&1) {
                Ordering::Greater => {
                    return Err(Error::MultipleUniqueCandidates {
                        row,
                        col,
                        candidates: d,
                    })
                }
                Ordering::Equal => {
                    let number = d.iter().next().unwrap();
                    let cell = board.get_mut(row, col).unwrap();
                    Rc::get_mut(cell).unwrap().set_number(*number);
                }
                Ordering::Less => {}
            }
        }
        Ok(())
    }
}
