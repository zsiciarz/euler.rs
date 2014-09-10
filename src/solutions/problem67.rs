use std::cmp;
use std::io::File;
use std::path::Path;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let path = Path::new("data/p067_triangle.txt");
    let contents = File::open(&path).read_to_string().ok().expect("Cannot read file");
    let mut lines = contents.as_slice().lines();
    let mut rows = Vec::new();
    for line in lines {
        let row = line.words().filter_map(from_str).collect::<Vec<int>>();
        rows.push(row);
    }
    for i in range(0u, rows.len() - 1).rev() {
        for j in range(0u, i + 1) {
            *rows.get_mut(i).get_mut(j) += cmp::max(rows[i + 1][j], rows[i + 1][j + 1]);
        }
    }
    Ok(rows[0][0])
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(73));
    }
}
