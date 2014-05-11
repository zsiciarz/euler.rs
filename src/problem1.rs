pub fn solution () -> int {
    use std::iter::AdditiveIterator;
    let mut numbers = range(1, 1000).filter(|i| i % 3 == 0 || i % 5  == 0);
    numbers.sum()
}
