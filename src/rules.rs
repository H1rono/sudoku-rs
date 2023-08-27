use crate::Board;

mod reduce_candidates;
mod single_candidate;
mod unique_candidate;

pub trait Rule {
    type Error: std::error::Error + 'static;
    fn apply(&self, board: &mut Board) -> Result<(), Self::Error>;
}

pub use reduce_candidates::ReduceCandidates;
pub use single_candidate::SingleCandidate;
pub use unique_candidate::UniqueCandidate;
