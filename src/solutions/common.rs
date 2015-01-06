use std::iter;
use std::iter::MultiplicativeIterator;
use std::mem;
use std::num::{Float, FromPrimitive, from_int};
use num;
use num::integer::Integer;
use slow_primes::Primes;

/// Converts an integer to a vector of digits (in reverse order).
pub fn digits<T: Clone + Integer + FromPrimitive>(n: T) -> Vec<T> {
    let mut digits = Vec::new();
    let mut q = n;
    let base: T = from_int(10).unwrap();
    while q > num::zero() {
        let r = q.clone() % base.clone();
        q = q / base.clone();
        digits.push(r);
    }
    digits
}

/// Converts a slice of digits (in reverse order) to an integer
pub fn undigits<T: Clone + Integer + FromPrimitive>(ds: &[T]) -> T {
    let base = from_int::<T>(10).unwrap();
    match ds {
        [ref i] => i.clone() + num::zero(),
        [ref i, ref j] => j.clone() * base + i.clone(),
        _ => {
            let mut res: T = num::zero();
            for (a, b) in ds.iter().zip(iter::count(0u, 1)) {
                res = res + a.clone() * num::pow(base.clone(), b);
            }
            res
        },
    }
}

/// Finds all proper divisors of an integer.
pub fn divisors(n: int) -> Vec<int> {
    let limit = (n as f32).sqrt().ceil() as int;
    let mut divisors = Vec::new();
    divisors.push(1);
    for i in range(2i, limit) {
        if n % i == 0 {
            divisors.push(i);
            divisors.push(n / i);
        }
    }
    divisors
}

pub fn num_divisors(n: int, primes: &Primes) -> int {
    let factors = primes.factor(n as uint);
    match factors {
        Ok(factors) => factors.into_iter().map(|(_, x)| x + 1).product() as int,
        Err(_) => 0i
    }
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

impl<T: Clone + Integer> Iterator for Fib<T> {
    type Item = T;

    /// Returns next value from the Fibonacci sequence.
    fn next(&mut self) -> Option<T> {
        let current = self.prev.clone() + self.current.clone();
        let prev = mem::replace(&mut self.prev, current);
        Some(mem::replace(&mut self.current, prev))
    }
}
