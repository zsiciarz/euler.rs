#![feature(slicing_syntax)]

extern crate num;
extern crate getopts;
extern crate slow_primes;

#[cfg(not(test))]
use getopts::{optopt, getopts};

#[cfg(not(test))]
use solutions::SolutionError;

mod solutions;

#[cfg(not(test))]
fn main() {
    let args = std::os::args();
    let program_name = args[0].clone();
    let opts = [
        optopt("p", "problem", "Problem number", "PROBLEM NUMBER"),
    ];
    let matches = match getopts(args.tail(), &opts) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) }
    };
    let problem_number = match matches.opt_str("p") {
        Some(p) => { p },
        None => {
            println!("Usage: {} --problem=<PROBLEM NUMBER>", program_name);
            return;
        }
    };
    match (problem_number[].parse::<int>()) {
        Some(problem_number) => match solutions::solution(problem_number) {
            Ok(x) => println!("{}", x),
            Err(e) => match e {
                SolutionError::NotImplemented => println!("Not implemented yet"),
                SolutionError::SolutionNotFound => println!("Solution not found"),
                SolutionError::MatchFailed => println!("Pattern matching failed"),
            }
        },
        None => println!("Invalid problem number")
    };
}
