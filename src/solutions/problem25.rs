use std::num;
use num::BigInt;

use super::{SolutionResult};
use super::common::{digits, Fib};

pub fn solution() -> SolutionResult {
    let mut fib: Fib<BigInt> = Fib::new();
    Ok(fib.map(|i| digits(i).len()).take_while(|&i| i < 1000).count() as int)
}
