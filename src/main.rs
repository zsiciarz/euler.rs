extern crate num;

mod solutions;

fn main() {
    let args = std::os::args();
    match args.as_slice() {
        [ref program_name] => println!("Usage: {} <PROBLEM NUMBER>", program_name),
        [_, ref problem, ..] => match from_str::<int>(problem.as_slice()) {
            Some(problem_number) => println!("{}", solutions::solution(problem_number)),
            None => println!("Invalid problem number"),
        },
        _ => unreachable!(),
    };
}
