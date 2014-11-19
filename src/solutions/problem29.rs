use std::collections::HashSet;
use std::num::from_uint;
use num;
use num::BigUint;

use super::{SolutionResult};

pub fn solution() -> SolutionResult {
    let mut s = HashSet::new();
    for a in range(2u, 101) {
        for b in range(2u, 101) {
            let a: BigUint = from_uint(a).unwrap();
            s.insert(num::pow(a, b));
        }
    }
    Ok(s.len() as int)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(83));
    }
}
