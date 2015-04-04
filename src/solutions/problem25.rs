use num;
use num::BigInt;
use num::traits::FromPrimitive;

use super::SolutionResult;
use super::common::Fib;

pub fn solution() -> SolutionResult {
    let base: BigInt = FromPrimitive::from_i32(10).unwrap();
    let threshold: BigInt = num::pow(base, 999);
    let fib: Fib<BigInt> = Fib::new();
    Ok(1 + fib.take_while(|i| *i < threshold).count() as i64)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(82));
    }
}
