extern crate num;

pub fn solution() -> int {
    range(1i, 20).fold(1, num::integer::lcm)
}
