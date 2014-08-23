use super::{SolutionResult, MatchFailed};
use slow_primes::Primes;

pub fn solution() -> SolutionResult {
    let primes = Primes::sieve(1000000);
    match primes.primes().take(10001).last() {
        Some(p) => Ok(p as int),
        None => Err(MatchFailed),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(43));
    }
}
