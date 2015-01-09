use std::num::{self, ToPrimitive};
use num::{BigInt, One, Zero};

use super::{SolutionResult, SolutionError};
use super::common::digits;

pub fn solution() -> SolutionResult {
    let one: BigInt = One::one();
    let zero: BigInt = Zero::zero();
    let fac100 = range(1i32, 101).map(|x| num::from_i32::<BigInt>(x).unwrap()).fold(one, |acc, x| acc * x);
    match digits(fac100).into_iter().fold(zero, |acc, x| acc + x).to_i32() {
        Some(x) => Ok(x),
        None => Err(SolutionError::MatchFailed),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(48));
    }
}
