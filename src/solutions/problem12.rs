use super::SolutionResult;
use super::common::num_divisors;
use slow_primes::Primes;

pub fn solution() -> SolutionResult {
    let mut n = 1;
    let primes = Primes::sieve(100000);
    loop {
        let triangular = n * (n + 1) / 2;
        if num_divisors(triangular, &primes) > 500 {
            return Ok(triangular);
        }
        n += 1;
    }
}
