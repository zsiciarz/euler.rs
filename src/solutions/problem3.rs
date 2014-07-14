use super::SolutionResult;
use super::common::prime_factors;

pub fn solution() -> SolutionResult {
    match prime_factors(600851475143).last() {
        None => Ok(0),
        Some(&x) => Ok(x)
    }
}
