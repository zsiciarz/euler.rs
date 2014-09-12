use super::SolutionResult;

/// Finds all proper divisors of an integer.
fn divisors(n: int) -> Vec<int> {
    let limit = (n as f32).sqrt().ceil() as int;
    let mut divisors = Vec::new();
    divisors.push(1);
    for i in range(2i, limit) {
        if n % i == 0 {
            divisors.push(i);
            divisors.push(n / i);
        }
    }
    divisors
}

pub fn solution() -> SolutionResult {
    Ok(21)
}
