use std::collections::HashMap;

pub fn prime_factors(n: int) -> Vec<int> {
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

pub fn prime_factor_groups(n: int) -> HashMap<int, int> {
    let mut powers = HashMap::new();
    for factor in prime_factors(n).move_iter() {
        powers.insert_or_update_with(factor, 1i, |_, v| *v += 1);
    }
    powers
}

pub fn num_divisors(n: int) -> int {
    use std::iter::MultiplicativeIterator;
    prime_factor_groups(n).values().map(|&x| x + 1).product()
}