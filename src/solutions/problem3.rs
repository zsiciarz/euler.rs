use super::common::prime_factors;

pub fn solution() -> int {
    match prime_factors(600851475143).last() {
        None => 0,
        Some(&x) => x
    }
}
