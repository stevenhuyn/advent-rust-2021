use std::env;

mod day1;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    let part = args[2].chars().next().unwrap();
    run(day, part);
}

fn run(day: i32, part: char) {
    match day {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        _ => println!("bruh"),
    };
}
