use std::fs::File;
use std::io::Read;

use super::SolutionResult;

fn is_triangular(n: u32) -> bool {
    let square = (8 * n + 1) as f32;
    square.sqrt().fract() == 0f32
}

fn word_value(name: &str) -> u32 {
    name.chars()
        .filter_map(|c| {
            if c == '\"' {
                None
            } else {
                Some((c as u8 - 64u8) as u32)
            }
        })
        .fold(0, |acc, x| acc + x)
}

pub fn solution() -> SolutionResult {
    let path = "data/p042_words.txt";
    let mut contents = String::new();
    File::open(path).unwrap().read_to_string(&mut contents).expect("Cannot read file");
    let names = contents[..].split(',');
    Ok(names.filter(|&x| is_triangular(word_value(x))).count() as i64)
}

test_solution!(62);
