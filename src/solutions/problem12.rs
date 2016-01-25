use super::SolutionResult;
use super::common::num_divisors;
use primal::Sieve;

pub fn solution() -> SolutionResult {
    let mut n = 1i64;
    let primes = Sieve::new(100000);
    loop {
        let triangular = n * (n + 1) / 2;
        if num_divisors(triangular, &primes) > 500 {
            return Ok(triangular);
        }
        n += 1;
    }
}

test_solution!(6500, 10000);
