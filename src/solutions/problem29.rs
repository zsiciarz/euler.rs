use std::collections::HashSet;
use std::num::from_u32;
use num;
use num::BigUint;

use super::{SolutionResult};

pub fn solution() -> SolutionResult {
    let mut s = HashSet::new();
    for a in range(2u32, 101) {
        for b in range(2, 101) {
            let a: BigUint = from_u32(a).unwrap();
            s.insert(num::pow(a, b));
        }
    }
    Ok(s.len() as i32)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(83));
    }
}
