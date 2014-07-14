use super::SolutionResult;
use std::iter::AdditiveIterator;
use std::num;

pub fn solution() -> SolutionResult {
    Ok(num::pow(range(1i, 101).sum(), 2) - range(1i, 101).map(|x| num::pow(x, 2)).sum())
}
