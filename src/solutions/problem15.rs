use super::SolutionResult;


pub fn solution() -> SolutionResult {
    let mut row = vec![1i64];
    for _ in (0..40) {
        let mut new_row = row.clone();
        new_row.insert(0, 0);
        row.push(0);
        row = row.iter().zip(new_row.iter()).map(|(a, b)| a + b).collect();
    }
    Ok(row[20])
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(20));
    }
}

