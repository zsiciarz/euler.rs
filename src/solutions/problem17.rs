use super::SolutionResult;

pub fn count_letters(n: usize) -> usize {
    let low_digits: Vec<usize> = vec!["",
                                      "one",
                                      "two",
                                      "three",
                                      "four",
                                      "five",
                                      "six",
                                      "seven",
                                      "eight",
                                      "nine",
                                      "ten",
                                      "eleven",
                                      "twelve",
                                      "thirteen",
                                      "fourteen",
                                      "fifteen",
                                      "sixteen",
                                      "seventeen",
                                      "eighteen",
                                      "nineteen"]
                                     .iter()
                                     .map(|&s| s.len())
                                     .collect();
    let tens: Vec<usize> = vec!["twenty", "thirty", "forty", "fifty", "sixty", "seventy",
                                "eighty", "ninety"]
                               .iter()
                               .map(|&s| s.len())
                               .collect();
    let (d10, m10) = (n / 10, n % 10);
    let (d100, m100) = (n / 100, n % 100);
    match n {
        _ if n < 20 => low_digits[n],
        _ if n < 100 => tens[d10 - 2] + count_letters(m10),
        _ if m100 == 0 => low_digits[d100] + "hundred".len(),
        _ => low_digits[d100] + "hundredand".len() + count_letters(m100),
    }
}

pub fn solution() -> SolutionResult {
    let answer = "onethousand".len() + (1..1000).fold(0, |acc, x| acc + count_letters(x));
    Ok(answer as i64)
}

test_solution!(24);
