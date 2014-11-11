use std::collections::HashSet;
use std::iter::AdditiveIterator;

use super::SolutionResult;
use super::common::undigits;

fn pandigital_products() -> HashSet<uint> {
    let v = [1u, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut perms = v.permutations();
    let mut result = HashSet::new();
    for p in perms {
        for i in range(1u, 6) {
            for j in range(i + 1, 9u) {
                let x = undigits(p[..i]);
                let y = undigits(p[i..j]);
                let z = undigits(p[j..]);
                if x == y * z {
                    result.insert(x);
                }
            }
        }
    }
    result
}

pub fn solution() -> SolutionResult {
    Ok(pandigital_products().into_iter().sum() as int)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(28));
    }
}
