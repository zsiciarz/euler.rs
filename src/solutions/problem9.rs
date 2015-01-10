use super::{SolutionResult, SolutionError};

fn is_pythagorean(a2: i64, b2: i64, c2: i64) -> bool {
    (a2 + b2 == c2) || (b2 + c2 == a2) || (a2 + c2 == b2)
}

pub fn solution() -> SolutionResult {
    for a in range(1i64, 500) {
        for b in range(a, a + 1000) {
            if b > 500 {
                continue;
            }
            let c = 1000 - (a + b);
            if is_pythagorean(a * a, b * b, c * c) {
                return Ok(a * b * c);
            }
        }
    }
    Err(SolutionError::SolutionNotFound)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 10000), Ok(5000));
    }
}
