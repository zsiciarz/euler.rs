use std::collections::HashMap;
use std::iter::AdditiveIterator;

use super::SolutionResult;
use super::common::divisors;

fn sum_divisors(n: i64) -> i64 {
    divisors(n).into_iter().sum()
}

pub fn solution() -> SolutionResult {
    let mut divisors_map = HashMap::new();
    for i in 2..10000 {
        divisors_map.insert(i, sum_divisors(i));
    }
    let mut sum = 0i64;
    for (key, value) in divisors_map.iter() {
        if *key != *value && divisors_map.get(value) == Some(key) {
            sum += *value;
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(26));
    }
}
