use permutohedron::Heap;
use primal::Sieve;

use super::SolutionResult;
use super::common::undigits;

fn pandigitals(n: usize) -> Vec<usize> {
    let mut v = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut heap = Heap::new(&mut v[..n]);
    let mut result = Vec::new();
    while let Some(p) = heap.next_permutation() {
        result.push(undigits(&p[..]));
    }
    result
}

pub fn solution() -> SolutionResult {
    let sieve = Sieve::new(9999999);
    let primes = (3..8).flat_map(|n| pandigitals(n).into_iter()).filter(|&x| sieve.is_prime(x));
    Ok(primes.max().unwrap() as i64)
}

test_solution!(13);
