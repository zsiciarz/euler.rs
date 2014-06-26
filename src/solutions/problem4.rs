use std::num;

/// Converts an integer to a vector of digits (in reverse order).
fn digits<T: Int + num::FromPrimitive>(n: T) -> Vec<T> {
    let mut digits = Vec::new();
    let mut q = n;
    let base: T = num::from_int(10).unwrap();
    while q > num::zero() {
        let r = q % base;
        q = q / base;
        digits.push(r);
    }
    digits
}

fn is_palindromic(n: int) -> bool {
    let digits = digits(n);
    let mut reversed = digits.clone();
    reversed.reverse();
    digits == reversed
}

pub fn solution() -> int {
    let mut result = 0;
    for x in range(100i, 999) {
        for y in range(100i, 999) {
            let product = x * y;
            if is_palindromic(product) && product > result {
                result = product;
            }
        }
    }
    result
}
