use num::rational::Ratio;

use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let mut r = Ratio::from_integer(1i64);
    for x in 1i64..10 {
        for y in 1i64..10 {
            for z in 1i64..10 {
                if x != y && 9 * x * z + y * z == 10 * x * y {
                    r = r * Ratio::new(10 * x + y, 10 * y + z);
                }
            }
        }
    }
    Ok(*r.denom())
}

test_solution!(0);
