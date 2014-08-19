extern crate num;
extern crate getopts;
extern crate slow_primes;

use getopts::{optopt, getopts};
use solutions::{NotImplemented, SolutionNotFound, MatchFailed};

mod solutions;

fn main() {
    let args = std::os::args();
    let program_name = args[0].clone();
    let opts = [
        optopt("p", "problem", "Problem number", "PROBLEM NUMBER"),
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => { fail!(f.to_string()) }
    };
    let problem_number = match matches.opt_str("p") {
        Some(p) => { p },
        None => {
            println!("Usage: {} <PROBLEM NUMBER>", program_name);
            return;
        }
    };
    match from_str::<int>(problem_number.as_slice()) {
        Some(problem_number) => match solutions::solution(problem_number) {
            Ok(x) => println!("{}", x),
            Err(e) => match e {
                NotImplemented => println!("Not implemented yet"),
                SolutionNotFound => println!("Solution not found"),
                MatchFailed => println!("Pattern matching failed"),
            }
        },
        None => println!("Invalid problem number")
    };
}
