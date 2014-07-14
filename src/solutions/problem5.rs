use num::integer::lcm;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok(range(1i, 20).fold(1, lcm))
}
