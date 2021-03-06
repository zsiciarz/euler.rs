use std::mem;
use num;
use num::integer::Integer;
use num::traits::FromPrimitive;
use primal::Sieve;

/// Converts an integer to a vector of digits (in reverse order).
pub fn digits<T: Clone + Integer + FromPrimitive>(n: T) -> Vec<T> {
    let mut digits = Vec::new();
    let mut q = n;
    let base: T = FromPrimitive::from_i64(10).unwrap();
    while q > num::zero() {
        let r = q.clone() % base.clone();
        q = q / base.clone();
        digits.push(r);
    }
    digits
}

/// Converts a slice of digits (in reverse order) to an integer
pub fn undigits<T: Clone + Integer + FromPrimitive>(ds: &[T]) -> T {
    let base: T = FromPrimitive::from_i64(10).unwrap();
    match ds.len() {
        1 => ds[0].clone() + num::zero(),
        2 => ds[1].clone() * base + ds[0].clone(),
        _ => {
            let mut res: T = num::zero();
            for (a, b) in ds.iter().zip(0..) {
                res = res + a.clone() * num::pow(base.clone(), b);
            }
            res
        }
    }
}

/// Finds all proper divisors of an integer.
pub fn divisors(n: i64) -> Vec<i64> {
    let limit = (n as f32).sqrt().ceil() as i64;
    let mut divisors = Vec::new();
    divisors.push(1i64);
    for i in 2..(limit) {
        if n % i == 0 {
            divisors.push(i);
            divisors.push(n / i);
        }
    }
    divisors
}

pub fn num_divisors(n: i64, primes: &Sieve) -> i64 {
    let factors = primes.factor(n as usize);
    match factors {
        Ok(factors) => factors.into_iter().fold(1, |acc, (_, x)| acc * (x + 1)) as i64,
        Err(_) => 0i64,
    }
}

/// An infinite generator of Fibonacci sequence.
pub struct Fib<T> {
    prev: T,
    current: T,
}

impl<T: Integer> Fib<T> {
    pub fn new() -> Fib<T> {
        Fib {
            prev: num::one(),
            current: num::one(),
        }
    }
}

impl<T: Clone + Integer> Iterator for Fib<T> {
    type Item = T;

    /// Returns next value from the Fibonacci sequence.
    fn next(&mut self) -> Option<T> {
        let current = self.prev.clone() + self.current.clone();
        let prev = mem::replace(&mut self.prev, current);
        Some(mem::replace(&mut self.current, prev))
    }
}
