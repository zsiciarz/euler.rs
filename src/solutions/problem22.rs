use std::fs::File;
use std::io::Read;

use super::SolutionResult;

fn name_value(name: &str) -> u32 {
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
    let path = "data/p022_names.txt";
    let mut contents = String::new();
    File::open(path).unwrap().read_to_string(&mut contents).expect("Cannot read file");
    let mut names: Vec<&str> = contents[..].split(',').collect();
    names.sort();
    Ok(names.into_iter()
            .enumerate()
            .map(|(i, name)| (i as u32 + 1) * name_value(name))
            .fold(0, |acc, x| acc + x) as i64)
}

test_solution!(82);
