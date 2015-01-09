use std::iter::AdditiveIterator;

use super::SolutionResult;
use super::common::Fib;

pub fn solution() -> SolutionResult {
    let fib: Fib<i32> = Fib::new();
    Ok(fib.take_while(|&i| i <= 4000000).filter(|&i| i % 2 == 0).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(32));
    }
}
