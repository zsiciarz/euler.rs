use super::SolutionResult;
use num::integer::lcm;

pub fn solution() -> SolutionResult {
    Ok(range(1i, 20).fold(1, lcm))
}
