use super::SolutionResult;

fn is_pythagorean(a2: int, b2: int, c2: int) -> bool {
    (a2 + b2 == c2) || (b2 + c2 == a2) || (a2 + c2 == b2)
}

pub fn solution() -> SolutionResult {
    for a in range(1i, 500) {
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
    Ok(0)
}
