#![feature(test)]

extern crate test;

use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    let part = args[2].parse::<i32>().unwrap();
    run(day, part);
}

fn run(day: i32, part: i32) {
    match day {
        1 => day1::run(part),
        2 => day2::run(part),
        3 => day3::run(part),
        4 => day4::run(part),
        5 => day5::run(part),
        6 => day6::run(part),
        7 => day7::run(part),
        _ => println!("bruh"),
    };
}
