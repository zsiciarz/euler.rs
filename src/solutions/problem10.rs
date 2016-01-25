use super::SolutionResult;
use primal::Sieve;

pub fn solution() -> SolutionResult {
    let primes = Sieve::new(2000000);
    Ok(primes.primes_from(0).take_while(|&p| p < 2000000).fold(0, |acc, x| acc + x) as i64)
}

test_solution!(22);
