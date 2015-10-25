use super::SolutionResult;

pub fn solution() -> SolutionResult {
    Ok((1i64..101).fold(0, |acc, x| acc + x).pow(2) -
       (1i64..101).map(|x| x.pow(2)).fold(0, |acc, x| acc + x))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(50));
    }
}
