use super::SolutionResult;

fn is_natural(x: f64) -> bool {
    (x - x.round()).abs() < 1e-6
}

fn is_pentagonal(x: i64) -> bool {
    let x = x as f64;
    is_natural(((24.0 * x + 1.0).sqrt() + 1.0) / 6.0)
}

fn hexagonal(n: i64) -> i64 {
    n * (2 * n - 1)
}

pub fn solution() -> SolutionResult {
    let x = (144..100000).map(hexagonal).filter(|&x| is_pentagonal(x)).take(1).next();
    Ok(x.unwrap())
}

test_solution!(5);
