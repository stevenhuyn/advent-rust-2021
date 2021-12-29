use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve_p1() {
    let mut crabs = read_input("input/day7.txt").unwrap();

    crabs.sort();
    let median;
    if crabs.len() % 2 == 0 {
        median = (crabs[crabs.len() / 2] + crabs[(crabs.len() / 2) - 1]) / 2;
    } else {
        median = crabs[crabs.len() / 2];
    }
    let fuel: i32 = crabs.iter().map(|v| i32::abs(v - median)).sum();

    println!("{:?}", fuel);
}

fn solve_p2() {
    let crabs = read_input("input/day7.txt").unwrap();

    let &min = crabs.iter().min().unwrap();
    let &max = crabs.iter().max().unwrap();

    let min_inv_triangle = inverse_pos_triangle(min as f32);
    let max_inv_triangle = inverse_pos_triangle(max as f32);

    let target = (max_inv_triangle - min_inv_triangle).round() as i32;

    let fuel: i32 = crabs.iter().map(|v| triangle(i32::abs(v - target))).sum();

    println!("{}", fuel);
}

// Use cached crate or cache by hand?
fn triangle(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn inverse_pos_triangle(n: f32) -> f32 {
    -1_f32 + (1_f32 + 8_f32 * n).sqrt() / 2_f32
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
