use super::SolutionResult;

/// An infinite generator of Fibonacci sequence.
struct Fib {
    prev: int,
    current: int,
}

impl Fib {
    fn new() -> Fib {
        Fib {prev: 1, current: 1}
    }
}

impl Iterator<int> for Fib {
    /// Returns next value from the Fibonacci sequence.
    fn next(&mut self) -> Option<int> {
        let prev = self.prev;
        self.prev = self.current;
        self.current = prev + self.current;
        Some(self.prev)
    }
}

pub fn solution() -> SolutionResult {
    use std::iter::AdditiveIterator;
    Ok(Fib::new().take_while(|&i| i <= 4000000).filter(|&i| i % 2 == 0).sum())
}
