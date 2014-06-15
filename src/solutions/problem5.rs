extern crate num;

pub fn solution() -> int {
    range(1, 20).fold(1, num::integer::lcm)
}
