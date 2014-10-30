use num::rational::Ratio;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let mut r = Ratio::new(1i, 1);
    for x in range(1i, 10) {
        for y in range(1i, 10) {
            for z in range(1i, 10) {
                if x != y && 9 * x * z + y * z == 10 * x * y {
                    r = r * Ratio::new(10 * x + y, 10 * y + z);
                }
            }
        }
    }
    Ok(*r.denom())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 10i), Ok(0));
    }
}
