use std::collections::HashMap;
use std::iter::AdditiveIterator;

use super::SolutionResult;
use super::common::divisors;

fn sum_divisors(n: int) -> int {
    divisors(n).into_iter().sum()
}

pub fn solution() -> SolutionResult {
    let mut divisors_map = HashMap::new();
    for i in range(2i, 10000) {
        divisors_map.insert(i, sum_divisors(i));
    }
    let mut sum = 0i;
    for (key, value) in divisors_map.iter() {
        if *key != *value && divisors_map.find(value) == Some(key) {
            sum += *value;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(26));
    }
}
