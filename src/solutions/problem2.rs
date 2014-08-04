use std::iter::AdditiveIterator;
use std::mem;
use std::num;

use super::SolutionResult;

/// An infinite generator of Fibonacci sequence.
struct Fib {
    prev: int,
    current: int,
}

impl Fib {
    fn new() -> Fib {
        Fib {prev: num::one(), current: num::one()}
    }
}

impl Iterator<int> for Fib {
    /// Returns next value from the Fibonacci sequence.
    fn next(&mut self) -> Option<int> {
        let current = self.prev + self.current;
        let prev = mem::replace(&mut self.prev, current);
        Some(mem::replace(&mut self.current, prev))
    }
}

pub fn solution() -> SolutionResult {
    Ok(Fib::new().take_while(|&i| i <= 4000000).filter(|&i| i % 2 == 0).sum())
}
