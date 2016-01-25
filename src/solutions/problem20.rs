use num::{BigInt, One, Zero};
use num::traits::{FromPrimitive, ToPrimitive};

use super::{SolutionResult, SolutionError};
use super::common::digits;

pub fn solution() -> SolutionResult {
    let one: BigInt = One::one();
    let zero: BigInt = Zero::zero();
    let fac100 = (1i64..101)
                     .map(|x| -> BigInt { FromPrimitive::from_i64(x).unwrap() })
                     .fold(one, |acc, x| acc * x);
    match ToPrimitive::to_i64(&digits(fac100).iter().fold(zero, |acc, x| acc + x)) {
        Some(x) => Ok(x),
        None => Err(SolutionError::MatchFailed),
    }
}

test_solution!(48);
