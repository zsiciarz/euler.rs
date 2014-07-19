use std::iter::AdditiveIterator;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok(range(1i, 1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum())
}
