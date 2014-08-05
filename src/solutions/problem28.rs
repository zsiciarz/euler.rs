use super::{SolutionResult};

fn sum_diagonals(level: int) -> (int, int) {
    if level == 1 {
        return (1, 1);
    }
    let step = 2 * (level - 1);
    let (prevSum, prevValue) = sum_diagonals(level - 1);
    (prevSum + 4 * prevValue + 10 * step, prevValue + 4 * step)
}

pub fn solution() -> SolutionResult {
    let (sum, value) = sum_diagonals(501);
    Ok(sum)
}
