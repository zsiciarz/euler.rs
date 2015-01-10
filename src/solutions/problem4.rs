use super::SolutionResult;
use super::common::digits;

fn is_palindromic(n: i64) -> bool {
    let digits = digits(n);
    let mut reversed = digits.clone();
    reversed.reverse();
    digits == reversed
}

pub fn solution() -> SolutionResult {
    let mut result = 0;
    for x in range(100i64, 999) {
        for y in range(100i64, 999) {
            let product = x * y;
            if is_palindromic(product) && product > result {
                result = product;
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(9));
    }
}
