use std::cmp;
use std::old_io::File;
use std::old_path::Path;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let path = Path::new("data/p067_triangle.txt");
    let contents = File::open(&path).read_to_string().ok().expect("Cannot read file");
    let mut lines = contents[].lines();
    let mut rows = Vec::new();
    for line in lines {
        let row = line.words().filter_map(|s| s.parse().ok()).collect::<Vec<i64>>();
        rows.push(row);
    }
    for i in (0..rows.len() - 1).rev() {
        for j in 0..i + 1 {
            (*rows[i])[j] += cmp::max(rows[i + 1][j], rows[i + 1][j + 1]);
        }
    }
    Ok(rows[0][0])
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(73));
    }
}
