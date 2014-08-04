use std::collections::HashMap;
use std::iter::MultiplicativeIterator;
use std::mem;
use std::num;
use num::integer::Integer;

/// Converts an integer to a vector of digits (in reverse order).
pub fn digits<T: Integer + num::FromPrimitive>(n: T) -> Vec<T> {
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
        powers.insert_or_update_with(factor, 1, |_, v| *v += 1);
    }
    powers
}

pub fn num_divisors(n: int) -> int {
    prime_factor_groups(n).values().map(|&x| x + 1).product()
}

/// An infinite generator of Fibonacci sequence.
pub struct Fib<T> {
    prev: T,
    current: T,
}

impl<T: Integer> Fib<T> {
    pub fn new() -> Fib<T> {
        Fib {prev: num::one(), current: num::one()}
    }
}

impl<T: Integer> Iterator<T> for Fib<T> {
    /// Returns next value from the Fibonacci sequence.
    fn next(&mut self) -> Option<T> {
        let current = self.prev + self.current;
        let prev = mem::replace(&mut self.prev, current);
        Some(mem::replace(&mut self.current, prev))
    }
}
