use super::common::digits;
use super::SolutionResult;

fn factorial(n: i64) -> i64 {
    (2..n + 1).fold(1, |acc, x| acc * x)
}

pub fn solution() -> SolutionResult {
    Ok((3i64..2540160)
           .filter(|&x| x == digits(x).into_iter().map(factorial).fold(0, |acc, x| acc + x))
           .fold(0, |acc, x| acc + x))
}

test_solution!(30);
