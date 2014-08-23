use std::num;
use std::iter::AdditiveIterator;

use super::{SolutionResult};
use super::common::digits;

fn sum_powers(pow: uint, x: int) -> int {
    digits(x).move_iter().map(|i| num::pow(i, pow)).sum()
}

pub fn solution() -> SolutionResult {
    Ok(range(2i, 1234567).filter(|&x| x == sum_powers(5, x)).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(39));
    }
}
