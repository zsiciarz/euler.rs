#[macro_export]
macro_rules! test_solution {
    ($e:expr, $digits:expr) => (
        #[cfg(test)]
        mod test {
            #[test]
            fn test_solution() {
                assert_eq!(super::solution().map(|s| s % $digits), Ok($e));
            }
        }
    );
    ($e:expr) => (test_solution!($e, 100););
}
