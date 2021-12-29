use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve(days: u128) {
    let lanternfish = read_input("input/day6.txt").unwrap();

    let mut school: HashMap<u128, u128> = HashMap::new();
    for timer in 0..9 {
        school.insert(timer, 0);
    }

    for v in lanternfish {
        if let Some(timer) = school.get_mut(&v) {
            *timer += 1;
        }
    }

    for _ in 0..days {
        school = step(school);
    }
    println!("{}", school.values().sum::<u128>());
}

fn solve_p1() {
    solve(80);
}

fn solve_p2() {
    solve(256);
}

fn step(school: HashMap<u128, u128>) -> HashMap<u128, u128> {
    let mut new_school: HashMap<u128, u128> = HashMap::new();

    for timer in 1..9 {
        new_school.insert(timer - 1, *school.get(&(timer)).unwrap());
    }

    let &parents = school.get(&0).unwrap();
    new_school.insert(8, *school.get(&0).unwrap());
    new_school.entry(6).and_modify(|v| {
        *v += parents;
    });

    new_school
}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

pub fn read_input(filename: &str) -> io::Result<Vec<u128>> {
    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    Ok(lines
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|v| v.parse::<u128>().unwrap())
        .collect())
}
