use std::fmt::Display;

use crate::{Board, Rule};

#[derive(Debug)]
pub struct Error(Box<dyn std::error::Error>);

impl From<Box<dyn std::error::Error>> for Error {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

struct WrappedRule<R: Rule>(R);

impl<R: Rule> Rule for WrappedRule<R> {
    type Error = Error;

    fn apply(&self, board: &mut Board) -> Result<(), Self::Error> {
        self.0
            .apply(board)
            .map_err(|error| std::convert::Into::<Box<dyn std::error::Error>>::into(error).into())
    }
}

pub struct Solver {
    rules: Vec<Box<dyn Rule<Error = Error>>>,
}

impl Solver {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn add_rule<R: Rule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(WrappedRule(rule)));
    }

    pub fn solve(&self, board: &mut Board) -> Result<(), Error> {
        let mut i = 0;
        while !board.fullfilled() && i < 100 {
            i += 1;
            for rule in &self.rules {
                rule.apply(board)?;
            }
        }
        Ok(())
    }
}

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

pub fn make_solver() -> Solver {
    use crate::rules::{ReduceCandidates, SingleCandidate, UniqueCandidate};
    let mut solver = Solver::new();
    solver.add_rule(SingleCandidate);
    solver.add_rule(ReduceCandidates);
    solver.add_rule(UniqueCandidate);
    solver
}
