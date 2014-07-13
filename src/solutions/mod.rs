mod common;
mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem8;
mod problem9;
mod problem12;
mod problem14;
mod problem16;
mod problem17;

pub fn solution(problem: int) -> int {
    println!("Running solution for problem #{}", problem);
    match problem {
        1 => problem1::solution(),
        2 => problem2::solution(),
        3 => problem3::solution(),
        4 => problem4::solution(),
        5 => problem5::solution(),
        6 => problem6::solution(),
        8 => problem8::solution(),
        9 => problem9::solution(),
        12 => problem12::solution(),
        14 => problem14::solution(),
        16 => problem16::solution(),
        17 => problem17::solution(),
        _  => -1,
    }
}
