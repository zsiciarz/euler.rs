use std::iter::AdditiveIterator;
use std::num;

pub fn solution() -> int {
    num::pow(range(1, 101).sum(), 2) - range(1, 101).map(|x| num::pow(x, 2)).sum()
}
