use std::collections::HashSet;
use num;
use num::BigUint;
use num::traits::FromPrimitive;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let mut s = HashSet::new();
    for a in 2u32..101 {
        for b in 2..101 {
            let a: BigUint = FromPrimitive::from_u32(a).unwrap();
            s.insert(num::pow(a, b));
        }
    }
    Ok(s.len() as i64)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(83));
    }
}
