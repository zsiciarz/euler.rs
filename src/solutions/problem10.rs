use std::iter::AdditiveIterator;

use super::{SolutionResult};
use slow_primes::Primes;

pub fn solution() -> SolutionResult {
    let primes = Primes::sieve(2000000);
    Ok(primes.primes().take_while(|&p| p < 2000000).sum() as int)
}