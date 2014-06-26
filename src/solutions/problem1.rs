pub fn solution() -> int {
    use std::iter::AdditiveIterator;
    range(1i, 1000).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}
