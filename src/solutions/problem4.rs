use super::SolutionResult;
use super::common::digits;

fn is_palindromic(n: int) -> bool {
    let digits = digits(n);
    let mut reversed = digits.clone();
    reversed.reverse();
    digits == reversed
}

pub fn solution() -> SolutionResult {
    let mut result = 0;
    for x in range(100i, 999) {
        for y in range(100i, 999) {
            let product = x * y;
            if is_palindromic(product) && product > result {
                result = product;
            }
        }
    }
    Ok(result)
}
