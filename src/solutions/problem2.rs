use std::iter::AdditiveIterator;
use std::mem;
use std::num;
use num::integer::Integer;

use super::SolutionResult;

/// An infinite generator of Fibonacci sequence.
struct Fib<T> {
    prev: T,
    current: T,
}

impl<T: Integer> Fib<T> {
    fn new() -> Fib<T> {
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

pub fn solution() -> SolutionResult {
    let fib: Fib<int> = Fib::new();
    Ok(fib.take_while(|&i| i <= 4000000).filter(|&i| i % 2 == 0).sum())
}
