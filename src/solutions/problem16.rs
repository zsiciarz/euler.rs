use std::num::{from_i32, ToPrimitive};
use num::{self, BigInt, Zero};

use super::{SolutionResult, SolutionError};
use super::common::digits;

pub fn solution() -> SolutionResult {
    let base: BigInt = from_i32(2).unwrap();
    let zero: BigInt = Zero::zero();
    match digits(num::pow(base, 1000)).into_iter().fold(zero, |acc, x| acc + x).to_i32() {
        Some(x) => Ok(x),
        None => Err(SolutionError::MatchFailed),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(66));
    }
}
