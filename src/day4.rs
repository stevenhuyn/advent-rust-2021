use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve_p1() {
    let (draws, games) = read_input("input/day4.txt").unwrap();
    println!("{:?}", draws);
    println!("{:?}", games);
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

pub fn read_input(filename: &str) -> io::Result<(Vec<i32>, Vec<Vec<Vec<i32>>>)> {
    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    let draws: Vec<i32> = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut games: Vec<Vec<Vec<i32>>> = Vec::new();
    for line in lines {
        let line_str = line.unwrap();
        if line_str == "" {
            games.push(Vec::new());
            continue;
        }

        let row = line_str
            .trim()
            .split_whitespace()
            .map(|n| {
                println!("Test:{}", n);
                n.parse::<i32>().unwrap()
            })
            .collect();

        games.last_mut().unwrap().push(row);
    }

    Ok((draws, games))
}
