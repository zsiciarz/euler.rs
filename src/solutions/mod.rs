mod common;

#[macro_use]
mod macros;

mod problem1;
mod problem2;
mod problem3;
mod problem4;
mod problem5;
mod problem6;
mod problem7;
mod problem8;
mod problem9;
mod problem10;
mod problem12;
mod problem13;
mod problem14;
mod problem15;
mod problem16;
mod problem17;
mod problem18;
mod problem19;
mod problem20;
mod problem21;
mod problem22;
mod problem25;
mod problem28;
mod problem29;
mod problem30;
mod problem32;
mod problem33;
mod problem34;
mod problem40;
mod problem41;
mod problem42;
mod problem45;
mod problem67;

#[derive(PartialEq, Debug)]
pub enum SolutionError {
    #[cfg(not(test))]
    NotImplemented,
    SolutionNotFound,
    MatchFailed,
}

pub type SolutionResult = Result<i64, SolutionError>;

#[cfg(not(test))]
pub fn solution(problem: isize) -> SolutionResult {
    println!("Running solution for problem #{}", problem);
    match problem {
        1 => problem1::solution(),
        2 => problem2::solution(),
        3 => problem3::solution(),
        4 => problem4::solution(),
        5 => problem5::solution(),
        6 => problem6::solution(),
        7 => problem7::solution(),
        8 => problem8::solution(),
        9 => problem9::solution(),
        10 => problem10::solution(),
        12 => problem12::solution(),
        13 => problem13::solution(),
        14 => problem14::solution(),
        15 => problem15::solution(),
        16 => problem16::solution(),
        17 => problem17::solution(),
        18 => problem18::solution(),
        19 => problem19::solution(),
        20 => problem20::solution(),
        21 => problem21::solution(),
        22 => problem22::solution(),
        25 => problem25::solution(),
        28 => problem28::solution(),
        29 => problem29::solution(),
        30 => problem30::solution(),
        32 => problem32::solution(),
        33 => problem33::solution(),
        34 => problem34::solution(),
        40 => problem40::solution(),
        41 => problem41::solution(),
        42 => problem42::solution(),
        45 => problem45::solution(),
        67 => problem67::solution(),
        _ => Err(SolutionError::NotImplemented),
    }
}
