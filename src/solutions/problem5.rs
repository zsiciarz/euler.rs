use num::integer::lcm;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok((1i64..20).fold(1, lcm))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(60));
    }
}
