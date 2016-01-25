use num::integer::lcm;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok((1i64..20).fold(1, lcm))
}

test_solution!(60);
