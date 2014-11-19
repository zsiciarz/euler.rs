use std::iter::AdditiveIterator;
use std::num::from_int;
use num;
use num::BigInt;

use super::{SolutionResult, SolutionError};
use super::common::digits;

pub fn solution() -> SolutionResult {
    let base: BigInt = from_int(2).unwrap();
    match digits(num::pow(base, 1000)).into_iter().sum().to_int() {
        Some(x) => Ok(x),
        None => Err(SolutionError::MatchFailed),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(66));
    }
}
