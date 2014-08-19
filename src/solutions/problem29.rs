use std::collections::HashSet;
use std::num;
use num::BigInt;

use super::{SolutionResult};

pub fn solution() -> SolutionResult {
    let mut s = HashSet::new();
    for a in range(2u, 101) {
        for b in range(2u, 101) {
            let a: BigInt = num::from_uint(a).unwrap();
            s.insert(num::pow(a, b));
        }
    }
    Ok(s.len() as int)
}
