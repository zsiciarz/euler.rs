use super::SolutionResult;
use super::common::digits;

fn sum_powers(pow: u32, x: i64) -> i64 {
    digits(x).into_iter().map(|i| i.pow(pow)).fold(0, |acc, x| acc + x)
}

pub fn solution() -> SolutionResult {
    Ok((2i64..1234567).filter(|&x| x == sum_powers(5, x)).fold(0, |acc, x| acc + x))
}

test_solution!(39);
