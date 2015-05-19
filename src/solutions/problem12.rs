use super::SolutionResult;
use super::common::num_divisors;
use primal::Primes;

pub fn solution() -> SolutionResult {
    let mut n = 1i64;
    let primes = Primes::sieve(100000);
    loop {
        let triangular = n * (n + 1) / 2;
        if num_divisors(triangular, &primes) > 500 {
            return Ok(triangular);
        }
        n += 1;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 10000), Ok(6500));
    }
}
