use super::{SolutionResult, SolutionError};
use primal;
use primal::Sieve;

pub fn solution() -> SolutionResult {
    let (_, upper_bound) = primal::estimate_nth_prime(10001);
    let primes = Sieve::new(upper_bound as usize);
    match primes.primes_from(0).nth(10001 - 1) {
        Some(p) => Ok(p as i64),
        None => Err(SolutionError::MatchFailed),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(43));
    }
}
