use super::{SolutionResult};
use primal::Primes;

pub fn solution() -> SolutionResult {
    let primes = Primes::sieve(2000000);
    Ok(primes.primes().take_while(|&p| p < 2000000).fold(0, |acc, x| acc + x) as i64)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(22));
    }
}
