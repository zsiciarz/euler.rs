use super::{SolutionResult, SolutionError};
use primal::Sieve;

pub fn solution() -> SolutionResult {
    match Sieve::new(10000).factor(600851475143).unwrap().into_iter().last() {
        None => Err(SolutionError::MatchFailed),
        Some((p, _)) => Ok(p as i64),
    }
}

test_solution!(57);
