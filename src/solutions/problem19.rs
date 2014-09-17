use super::SolutionResult;


fn is_leap_year(year: int) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_february(year: int) -> int {
    if is_leap_year(year) {
        29
    } else {
        28
    }
}

fn year_days(year: int) -> Vec<int> {
    vec!(31, days_in_february(year), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31)
}

pub fn solution() -> SolutionResult {
    let days = range(1901i, 2001).flat_map(|year| year_days(year).into_iter());
    let num_sundays = days.scan(0, |acc: &mut int, x: int| {
        *acc = *acc + x;
        Some(*acc)
    }).filter(|&x| x % 7 == 0).count() + 1;
    Ok(num_sundays as int)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100i), Ok(71));
    }
}
