use std::io::File;
use std::iter::AdditiveIterator;
use std::num::Float;
use std::path::Path;

use super::SolutionResult;

fn is_triangular(n: uint) -> bool {
    let square = (8 * n + 1) as f32;
    square.sqrt().fract() == 0f32
}

fn word_value(name: &str) -> uint {
    name.chars().filter_map(|c| if c != '\"' {
        Some((c.to_ascii().to_byte() - 64u8) as uint)
    } else {
        None
    }).sum()
}

pub fn solution() -> SolutionResult {
    let path = Path::new("data/p042_words.txt");
    let contents = File::open(&path).read_to_string().ok().expect("Cannot read file");
    let names = contents[].split(',');
    Ok(names.filter(|&x| is_triangular(word_value(x))).count() as int)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(62));
    }
}
