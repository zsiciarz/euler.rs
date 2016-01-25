use std::collections::HashMap;

use super::SolutionResult;
use super::common::divisors;

fn sum_divisors(n: i64) -> i64 {
    divisors(n).into_iter().fold(0, |acc, x| acc + x)
}

pub fn solution() -> SolutionResult {
    let mut divisors_map = HashMap::new();
    for i in 2i64..10000 {
        divisors_map.insert(i, sum_divisors(i));
    }
    let mut sum = 0i64;
    for (key, value) in &divisors_map {
        if *key != *value && divisors_map.get(value) == Some(key) {
            sum += *value;
        }
    }
    Ok(sum)
}

test_solution!(26);
