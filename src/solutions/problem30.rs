use std::num::Int;
use std::iter::AdditiveIterator;

use super::{SolutionResult};
use super::common::digits;

fn sum_powers(pow: usize, x: i64) -> i64 {
    digits(x).into_iter().map(|i| i.pow(pow)).sum()
}

pub fn solution() -> SolutionResult {
    Ok((2..1234567).filter(|&x| x == sum_powers(5, x)).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(39));
    }
}
