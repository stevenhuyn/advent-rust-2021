use itertools::izip;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run(day: char) {
    match day {
        'a' => solve_p1(),
        'b' => solve_p2(),
        _ => println!("unknown day!"),
    }
}

fn solve_p1() {
    println!("p1 answer")
}

fn solve_p2() {
    println!("p2 answer")
}

pub fn read_input(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map(|a| a.unwrap().parse::<i32>().unwrap()).collect())
}
