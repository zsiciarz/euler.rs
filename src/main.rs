extern crate num;

use solutions::NotImplemented;

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
                }
            },
            None => println!("Invalid problem number"),
        },
        _ => unreachable!(),
    };
}
