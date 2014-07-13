use std::num::pow;
use num::BigInt;

use super::common::digits;

pub fn solution() -> int {
    use std::iter::AdditiveIterator;
    let two = from_str::<BigInt>("2").unwrap();
    match digits(pow(two, 1000)).move_iter().sum().to_int() {
        Some(x) => x,
        None => -1
    }
}
