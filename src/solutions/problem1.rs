use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok((1i64..1000).filter(|&i| i % 3 == 0 || i % 5 == 0).fold(0, |acc, x| acc + x))
}

test_solution!(68);
