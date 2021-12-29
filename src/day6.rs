use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve(days: i32) {
    let mut lanternfish = read_input("input/day6.txt").unwrap();

    for _ in 0..days {
        step(&mut lanternfish);
    }
    println!("{}", lanternfish.len());
}

fn solve_p1() {
    solve(80);
}

fn solve_p2() {
    solve(256);
}

const NEWBORN_TIMER: i32 = 8;
const TIMER: i32 = 6;
fn step(school: &mut Vec<i32>) {
    let mut newborn_count = 0;
    for fish_time in school.iter_mut() {
        *fish_time = if *fish_time == 0 {
            newborn_count += 1;
            TIMER
        } else {
            *fish_time - 1
        };
    }

    school.extend(vec![NEWBORN_TIMER; newborn_count]);
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
    let mut lines = BufReader::new(file).lines();

    Ok(lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect())
}
