use super::common::num_divisors;

pub fn solution() -> int {
    let mut n = 1;
    loop {
        let triangular = n * (n + 1) / 2;
        if num_divisors(triangular) > 500 {
            return triangular;
        }
        n += 1;
    }
}
