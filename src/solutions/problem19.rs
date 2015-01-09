use super::SolutionResult;


fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_february(year: i32) -> i32 {
    if is_leap_year(year) {
        29
    } else {
        28
    }
}

fn year_days(year: i32) -> Vec<i32> {
    vec!(31, days_in_february(year), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31)
}

pub fn solution() -> SolutionResult {
    let days = range(1901i32, 2001).flat_map(|year| year_days(year).into_iter());
    let num_sundays = days.scan(0, |acc: &mut i32, x: i32| {
        *acc = *acc + x;
        Some(*acc)
    }).filter(|&x| x % 7 == 0).count() + 1;
    Ok(num_sundays as i32)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_solution() {
        assert_eq!(super::solution().map(|s| s % 100), Ok(71));
    }
}
