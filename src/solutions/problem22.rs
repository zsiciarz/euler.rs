use std::old_io::File;
use std::iter::AdditiveIterator;
use std::path::Path;

use super::SolutionResult;

fn name_value(name: &str) -> u32 {
    name.chars().filter_map(|c| if c != '\"' {
        Some((c as u8 - 64u8) as u32)
    } else {
        None
    }).sum()
}

pub fn solution() -> SolutionResult {
    let path = Path::new("data/p022_names.txt");
    let contents = File::open(&path).read_to_string().ok().expect("Cannot read file");
    let mut names: Vec<&str> = contents[].split(',').collect();
    names.sort();
    Ok(names.into_iter().enumerate().map(|(i, name)| (i as u32 + 1) * name_value(name)).sum() as i64)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(82));
    }
}
