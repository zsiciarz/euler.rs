fn prime_factors(n: int) -> Vec<int> {
    let mut factors = Vec::new();
    let mut q = n;
    let mut p = 2;
    while p * p <= q {
        if q % p == 0 {
            factors.push(p);
            q /= p;
        }
        else {
            p += 1;
        }
    }
    if q > 1 {
        factors.push(q);
    }
    factors
}

pub fn solution() -> int {
    match prime_factors(600851475143).last() {
        None => 0,
        Some(&x) => x
    }
}
