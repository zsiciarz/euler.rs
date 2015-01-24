use super::{SolutionResult, SolutionError};
use slow_primes::Primes;

pub fn solution() -> SolutionResult {
    match Primes::sieve(10000).factor(600851475143).ok().unwrap().into_iter().last() {
        None => Err(SolutionError::MatchFailed),
        Some((p, _)) => Ok(p as i64)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(57));
    }
}
