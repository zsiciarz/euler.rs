use std::iter::{AdditiveIterator, MultiplicativeIterator};
use std::num;
use num::BigInt;

use super::{SolutionResult,MatchFailed};
use super::common::digits;

pub fn solution() -> SolutionResult {
    let fac100 = range(1i, 101).map(|x| num::from_int::<BigInt>(x).unwrap()).product();
    match digits(fac100).move_iter().sum().to_int() {
        Some(x) => Ok(x),
        None => Err(MatchFailed),
    }
}
