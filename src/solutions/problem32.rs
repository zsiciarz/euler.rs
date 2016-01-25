use std::collections::HashSet;

use permutohedron::Heap;

use super::SolutionResult;
use super::common::undigits;

fn pandigital_products() -> HashSet<i64> {
    let mut v = [1i64, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut heap = Heap::new(&mut v);
    let mut result = HashSet::new();
    while let Some(p) = heap.next_permutation() {
        for i in 1..6 {
            for j in i + 1..9 {
                let x = undigits(&p[..i]);
                let y = undigits(&p[i..j]);
                let z = undigits(&p[j..]);
                if x == y * z {
                    result.insert(x);
                }
            }
        }
    }
    result
}

pub fn solution() -> SolutionResult {
    Ok(pandigital_products().into_iter().fold(0, |acc, x| acc + x))
}

test_solution!(28);
