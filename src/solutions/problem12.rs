use super::SolutionResult;
use super::common::num_divisors;

pub fn solution() -> SolutionResult {
    let mut n = 1;
    loop {
        let triangular = n * (n + 1) / 2;
        if num_divisors(triangular) > 500 {
            return Ok(triangular);
        }
        n += 1;
    }
}
