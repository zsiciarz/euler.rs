use std::iter::AdditiveIterator;
use std::num::Int;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok(range(1i, 101).sum().pow(2) - range(1i, 101).map(|x| x.pow(2)).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(50));
    }
}
