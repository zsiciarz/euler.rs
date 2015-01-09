use super::{SolutionResult};

fn sum_diagonals(level: i32) -> (i32, i32) {
    if level == 1 {
        return (1, 1);
    }
    let step = 2 * (level - 1);
    let (prev_sum, prev_value) = sum_diagonals(level - 1);
    (prev_sum + 4 * prev_value + 10 * step, prev_value + 4 * step)
}

pub fn solution() -> SolutionResult {
    let (sum, _) = sum_diagonals(501);
    Ok(sum)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(1));
    }
}
