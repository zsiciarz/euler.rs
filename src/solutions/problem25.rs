use std::num;
use num::BigInt;

use super::{SolutionResult};
use super::common::Fib;

pub fn solution() -> SolutionResult {
    let base: BigInt = num::from_int(10).unwrap();
    let threshold: BigInt = num::pow(base, 999);
    let fib: Fib<BigInt> = Fib::new();
    Ok(1 + fib.take_while(|i| *i < threshold).count() as int)
}
