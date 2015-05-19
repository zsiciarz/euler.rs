use super::{SolutionResult, SolutionError};
use primal;
use primal::Primes;

pub fn solution() -> SolutionResult {
    let (_, upper_bound) = primal::estimate_nth_prime(10001);
    let primes = Primes::sieve(upper_bound as usize);
    match primes.primes().nth(10001 - 1) {
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
