use std::iter::{AdditiveIterator, MultiplicativeIterator};
use std::num;
use num::BigInt;

use super::{SolutionResult, SolutionError};
use super::common::digits;

pub fn solution() -> SolutionResult {
    let fac100 = range(1i, 101).map(|x| num::from_int::<BigInt>(x).unwrap()).product();
    match digits(fac100).into_iter().sum().to_int() {
        Some(x) => Ok(x),
        None => Err(SolutionError::MatchFailed),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(48));
    }
}
