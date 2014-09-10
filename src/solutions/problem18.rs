use std::cmp;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let mut rows = Vec::new();
    rows.push(vec![3i, 0, 0, 0]);
    rows.push(vec![7i, 4, 0, 0]);
    rows.push(vec![2i, 4, 6, 0]);
    rows.push(vec![8i, 5, 9, 3]);
    for i in range(0u, rows.len() - 1).rev() {
        for j in range(0u, i + 1) {
            *rows.get_mut(i).get_mut(j) += cmp::max(rows[i + 1][j], rows[i + 1][j + 1]);
        }
    }
    Ok(rows[0][0])
}
