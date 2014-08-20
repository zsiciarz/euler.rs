use std::io::{BufferedReader, File};
use std::path::Path;
use super::{SolutionResult};

pub fn solution() -> SolutionResult {
    let path = Path::new("data/p022_names.txt");
    let contents = File::open(&path).read_to_string().ok().expect("Cannot read file");
    let names: Vec<&str> = contents.as_slice().split(',').collect();
    Ok(22)
}
