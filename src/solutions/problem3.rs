use super::{SolutionResult, MatchFailed};
use slow_primes::Primes;

pub fn solution() -> SolutionResult {
    match Primes::sieve(10000).factor(600851475143).iter().last() {
        None => Err(MatchFailed),
        Some(&(p, _)) => Ok(p as int)
    }
}
