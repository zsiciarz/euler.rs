use slow_primes::Primes;

use super::SolutionResult;
use super::common::undigits;

fn pandigitals(n: usize) -> Vec<usize> {
    let v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v = &v[..n];
    let mut perms = v.permutations();
    let mut result = Vec::new();
    for p in perms {
        result.push(undigits(&p[]));
    }
    result
}

pub fn solution() -> SolutionResult {
    let sieve = Primes::sieve(9999999);
    let primes = range(3, 8).flat_map(|n| pandigitals(n).into_iter()).filter(|&x| sieve.is_prime(x));
    Ok(primes.max().unwrap() as i32)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(13));
    }
}
