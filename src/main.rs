mod solutions;

fn main() {
    let args = std::os::args();
    match args.as_slice() {
        [_, ref solution, ..] => println!("{}", solution),
        [ref program_name] => println!("Usage: {} <PROBLEM NUMBER>", program_name),
    };
    println!("{}", solutions::solution(14));
}
