use std::num::{from_int,pow};
use num::BigInt;

use super::common::digits;

pub fn solution() -> int {
    use std::iter::AdditiveIterator;
    let base: BigInt = from_int(2).unwrap();
    match digits(pow(base, 1000)).move_iter().sum().to_int() {
        Some(x) => x,
        None => -1
    }
}