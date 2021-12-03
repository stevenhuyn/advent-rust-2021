use itertools::izip;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve_p1() {
    println!("p1 answer")
}

fn solve_p2() {
    println!("p2 answer")
}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

pub fn read_input(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map(|a| a.unwrap().parse::<i32>().unwrap()).collect())
}