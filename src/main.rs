extern crate num;
extern crate getopts;
extern crate permutohedron;
extern crate primal;

#[cfg(not(test))]
use getopts::Options;

#[cfg(not(test))]
use solutions::SolutionError;

mod solutions;

#[cfg(not(test))]
fn main() {
    let args = std::env::args();
    let mut opts = Options::new();
    opts.optopt("p", "problem", "Problem number", "PROBLEM NUMBER");
    let matches = match opts.parse(args) {
        Ok(m) => {
            m
        }
        Err(f) => {
            panic!(f.to_string())
        }
    };
    let problem_number = match matches.opt_str("p") {
        Some(p) => {
            p
        }
        None => {
            println!("Usage: euler --problem=<PROBLEM NUMBER>");
            return;
        }
    };
    match problem_number.parse::<isize>() {
        Ok(problem_number) => match solutions::solution(problem_number) {
            Ok(x) => println!("{}", x),
            Err(e) => match e {
                SolutionError::NotImplemented => println!("Not implemented yet"),
                SolutionError::SolutionNotFound => println!("Solution not found"),
                SolutionError::MatchFailed => println!("Pattern matching failed"),
            },
        },
        Err(_) => println!("Invalid problem number"),
    }
}
