use super::{SolutionResult, MatchFailed};
use slow_primes::Primes;

pub fn solution() -> SolutionResult {
    let primes = Primes::sieve(1000000);
    match primes.primes().take(10001).last() {
        Some(p) => Ok(p as int),
        None => Err(MatchFailed),
    }
}
