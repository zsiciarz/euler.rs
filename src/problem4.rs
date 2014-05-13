fn digits (n: int) -> Vec<int> {
    let mut digits = Vec::new();
    let mut q = n;
    while q > 0 {
        let r = q % 10;
        q = q / 10;
        digits.push(r);
    }
    digits
}

fn isPalindromic(n: int) -> bool {
    let digits = digits(n);
    let mut reversed = digits.clone();
    reversed.reverse();
    digits == reversed
}

pub fn solution () -> int {
    let mut result = 0;
    for x in range(100, 999) {
        for y in range(100, 999) {
            let product = x * y;
            if isPalindromic(product) && product > result {
                result = product;
            }
        }
    }
    result
}
