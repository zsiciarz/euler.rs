use num::integer::lcm;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok(range(1i, 20).fold(1, lcm))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(60));
    }
}
