use super::common::digits;
use super::SolutionResult;

pub fn solution() -> SolutionResult {
    let mut i = 1i;
    let mut all_digits = vec![1i];
    loop {
        i += 1;
        let mut digs = digits(i);
        digs.reverse();
        all_digits.push_all(digs[]);
        if all_digits.len() > 999999 {
            break;
        }
    }
    Ok(all_digits[0] * all_digits[9] * all_digits[99] * all_digits[999] *
       all_digits[9999] * all_digits[99999] * all_digits[999999])
}
