use std::iter::AdditiveIterator;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok(range(1i32, 1000).filter(|&i| i % 3 == 0 || i % 5 == 0).sum())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(68));
    }
}
