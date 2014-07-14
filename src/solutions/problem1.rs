use super::SolutionResult;

pub fn solution() -> SolutionResult {
    use std::iter::AdditiveIterator;
    Ok(range(1i, 1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum())
}
