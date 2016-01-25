use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok((1i64..101).fold(0, |acc, x| acc + x).pow(2) -
       (1i64..101).map(|x| x.pow(2)).fold(0, |acc, x| acc + x))
}

test_solution!(50);
