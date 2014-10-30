use std::iter::AdditiveIterator;
use std::iter::MultiplicativeIterator;

use super::common::digits;
use super::SolutionResult;

fn factorial(n: int) -> int {
    return range(2, n + 1).product()
}

pub fn solution() -> SolutionResult {
    Ok(range(3i, 2540160).filter(|&x| x == digits(x).into_iter().map(factorial).sum()).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(30));
    }
}
