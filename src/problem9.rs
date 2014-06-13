fn is_pythagorean(a: int, b: int, c: int) -> bool {
    (a * a + b * b == c * c) ||
    (b * b + c * c == a * a) ||
    (a * a + c * c == b * b)
}

pub fn solution() -> int {
    for a in range(1, 1000) {
        for b in range(a, a + 1000) {
            if b > 500 {
                continue;
            }
            let c = 1000 - (a + b);
            if is_pythagorean(a, b, c) {
                return a * b * c;
            }
        }
    }
    0
}
