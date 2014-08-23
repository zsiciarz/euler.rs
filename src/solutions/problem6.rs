use std::iter::AdditiveIterator;
use std::num;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok(num::pow(range(1i, 101).sum(), 2) - range(1i, 101).map(|x| num::pow(x, 2)).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(50));
    }
}
