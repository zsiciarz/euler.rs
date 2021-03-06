use super::SolutionResult;
use super::common::Fib;

pub fn solution() -> SolutionResult {
    let fib: Fib<i64> = Fib::new();
    Ok(fib.take_while(|&i| i <= 4000000).filter(|&i| i % 2 == 0).fold(0, |acc, x| acc + x))
}

test_solution!(32);
