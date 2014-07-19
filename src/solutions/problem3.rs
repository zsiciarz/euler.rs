use super::{SolutionResult, MatchFailed};
use super::common::prime_factors;

pub fn solution() -> SolutionResult {
    match prime_factors(600851475143).last() {
        None => Err(MatchFailed),
        Some(&x) => Ok(x)
    }
}
