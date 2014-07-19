extern crate num;

use solutions::{NotImplemented, SolutionNotFound, MatchFailed};

mod solutions;

fn main() {
    let args = std::os::args();
    match args.as_slice() {
        [ref program_name] => println!("Usage: {} <PROBLEM NUMBER>", program_name),
        [_, ref problem, ..] => match from_str::<int>(problem.as_slice()) {
            Some(problem_number) => match solutions::solution(problem_number) {
                Ok(x) => println!("{}", x),
                Err(e) => match e {
                    NotImplemented => println!("Not implemented yet"),
                    SolutionNotFound => println!("Solution not found"),
                    MatchFailed => println!("Pattern matching failed"),
                }
            },
            None => println!("Invalid problem number"),
        },
        _ => unreachable!(),
    };
}
