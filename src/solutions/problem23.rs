use std::collections::BTreeSet;

use super::SolutionResult;
use super::common::divisors;

fn is_abundant(n: i64) -> bool {
    let sum = divisors(n).into_iter().fold(0, |acc, x| acc + x);
    sum > n
}

fn is_abundant_sum(n: i64, abundants: &BTreeSet<i64>) -> bool {
    for i in abundants.iter() {
        if *i > n {
            return false;
        }
        if abundants.contains(&(n - *i)) {
            return true;
        }
    }
    return false;
}

pub fn solution() -> SolutionResult {
    println!("{}", is_abundant(28));
    println!("{}", is_abundant(496));
    println!("{}", is_abundant(8128));
    let abundants: BTreeSet<i64> = (12..21824i64).filter(|&x| is_abundant(x)).collect();
    Ok((1..21824i64)
        .filter(|&x| !is_abundant_sum(x, &abundants))
        .fold(0, |acc, x| acc + x))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(71));
    }
}
