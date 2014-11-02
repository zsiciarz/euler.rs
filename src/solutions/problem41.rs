use std::iter;
use std::iter::AdditiveIterator;
use std::num;

use slow_primes::Primes;

use super::SolutionResult;

fn pandigitals(n: uint) -> Vec<uint> {
    let v = [1u, 2, 3, 4, 5, 6, 7, 8, 9];
    let v = v[..n];
    let mut perms = v.permutations();
    let mut result = Vec::new();
    for p in perms {
        result.push(undigits(p[]));
    }
    result
}

fn undigits(ds: &[uint]) -> uint {
    ds.iter().zip(iter::count(0, 1)).map(|(&a, b)| a * num::pow(10, b)).sum()
}

pub fn solution() -> SolutionResult {
    let sieve = Primes::sieve(9999999);
    let mut primes = range(3, 8).flat_map(|n| pandigitals(n).into_iter()).filter(|&x| sieve.is_prime(x));
    Ok(primes.max().unwrap() as int)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(13));
    }
}
