fn collatz_length(n: int) -> int {
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

pub fn solution() -> int {
    let mut max_length = 0;
    let mut max_index = 0;
    for i in range(1i, 1000000) {
        let length = collatz_length(i);
        if length > max_length {
            max_index = i;
            max_length = length;
        }
    }
    max_index
}
