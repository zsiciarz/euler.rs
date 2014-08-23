use std::io::File;
use std::iter::AdditiveIterator;
use std::path::Path;

use super::SolutionResult;

fn name_value(name: &str) -> uint {
    name.chars().filter_map(|c| if c != '\"' {
        Some((c.to_ascii().to_byte() - 64u8) as uint)
    } else {
        None
    }).sum()
}

pub fn solution() -> SolutionResult {
    let path = Path::new("data/p022_names.txt");
    let contents = File::open(&path).read_to_string().ok().expect("Cannot read file");
    let mut names: Vec<&str> = contents.as_slice().split(',').collect();
    names.sort();
    Ok(names.move_iter().enumerate().map(|(i, name)| (i + 1) * name_value(name)).sum() as int)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(82));
    }
}
