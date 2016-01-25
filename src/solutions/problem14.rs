use super::SolutionResult;

fn collatz_length(n: i64) -> i64 {
    let mut x = n;
    let mut i = 1;
    while x > 1 {
        x = match x % 2 {
            0 => x / 2,
            _ => 3 * x + 1,
        };
        i += 1;
    }
    i
}

pub fn solution() -> SolutionResult {
    let mut max_length = 0;
    let mut max_index = 0;
    for i in 1i64..1000000 {
        let length = collatz_length(i);
        if length > max_length {
            max_index = i;
            max_length = length;
        }
    }
    Ok(max_index)
}

test_solution!(99);
