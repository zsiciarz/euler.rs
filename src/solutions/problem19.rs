use super::SolutionResult;


fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_february(year: i64) -> i64 {
    if is_leap_year(year) {
        29
    } else {
        28
    }
}

fn year_days(year: i64) -> Vec<i64> {
    vec![31, days_in_february(year), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
}

pub fn solution() -> SolutionResult {
    let days = (1901i64..2001).flat_map(|year| year_days(year).into_iter());
    let num_sundays = days.scan(0, |acc: &mut i64, x: i64| {
                              *acc += x;
                              Some(*acc)
                          })
                          .filter(|&x| x % 7 == 0)
                          .count() + 1;
    Ok(num_sundays as i64)
}

test_solution!(71);
