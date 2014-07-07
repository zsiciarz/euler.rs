use std::num::div_rem;

pub fn count_letters(n: uint) -> uint {
    let low_digits: Vec<uint> = vec!(
        "", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine", "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen"
    ).iter().map(|&s| s.len()).collect();
    let tens: Vec<uint> = vec!(
        "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"
    ).iter().map(|&s| s.len()).collect();
    let (d10, m10) = div_rem(n, 10);
    let (d100, m100) = div_rem(n, 100);
    match n {
        _ if n < 20 => *low_digits.get(n),
        _ if n < 100 => *tens.get(d10 - 2) + count_letters(m10),
        _ if m100 == 0 => *low_digits.get(d100) + "hundred".len(),
        _ => *low_digits.get(d100) + "hundredand".len() + count_letters(m100),
    }
}

pub fn solution() -> int {
    let answer = "onethousand".len() + range(1u, 1000u).fold(0, |acc, x| acc + count_letters(x));
    answer as int
}