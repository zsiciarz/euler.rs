use super::{SolutionResult, SolutionError};

fn is_pythagorean(a2: i64, b2: i64, c2: i64) -> bool {
    (a2 + b2 == c2) || (b2 + c2 == a2) || (a2 + c2 == b2)
}

pub fn solution() -> SolutionResult {
    for a in 1i64..500 {
        for b in a..a + 1000 {
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

test_solution!(5000, 10000);
