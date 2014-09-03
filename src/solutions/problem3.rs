use super::{SolutionResult, MatchFailed};
use slow_primes::Primes;

pub fn solution() -> SolutionResult {
    match Primes::sieve(10000).factor(600851475143).unwrap().iter().last() {
        None => Err(MatchFailed),
        Some(&(p, _)) => Ok(p as int)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(57));
    }
}
