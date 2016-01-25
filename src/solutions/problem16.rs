use num::{self, BigInt, Zero};
use num::traits::{FromPrimitive, ToPrimitive};

use super::{SolutionResult, SolutionError};
use super::common::digits;

pub fn solution() -> SolutionResult {
    let base: BigInt = FromPrimitive::from_i64(2).unwrap();
    let zero: BigInt = Zero::zero();
    match digits(num::pow(base, 1000)).into_iter().fold(zero, |acc, x| acc + x).to_i64() {
        Some(x) => Ok(x),
        None => Err(SolutionError::MatchFailed),
    }
}

test_solution!(66);
