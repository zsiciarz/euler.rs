use std::cmp;
use std::fs::File;
use std::io::Read;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let path = "data/p067_triangle.txt";
    let mut contents = String::new();
    File::open(path).unwrap().read_to_string(&mut contents).expect("Cannot read file");
    let lines = contents[..].lines();
    let mut rows = Vec::new();
    for line in lines {
        let row = line.split(|c| c == ' ').filter_map(|s| s.parse().ok()).collect::<Vec<i64>>();
        rows.push(row);
    }
    for i in (0..rows.len() - 1).rev() {
        for j in 0..i + 1 {
            (*rows[i])[j] += cmp::max(rows[i + 1][j], rows[i + 1][j + 1]);
        }
    }
    Ok(rows[0][0])
}

test_solution!(73);
