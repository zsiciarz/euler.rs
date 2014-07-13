use num::integer::lcm;

pub fn solution() -> int {
    range(1i, 20).fold(1, lcm)
}
