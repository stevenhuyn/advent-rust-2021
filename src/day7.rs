use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn solve_p1() {
    let mut crabs = read_input("input/day7.txt").unwrap();

    let now = Instant::now();
    crabs.sort();
    let median;
    if crabs.len() % 2 == 0 {
        median = (crabs[crabs.len() / 2] + crabs[(crabs.len() / 2) - 1]) / 2;
    } else {
        median = crabs[crabs.len() / 2];
    }
    let fuel: i32 = crabs.iter().map(|v| i32::abs(v - median)).sum();

    println!("{:?}", fuel);
    println!("Finished in {:.5?}", now.elapsed());
}

// nlogn
fn solve_p2() {
    let crabs = read_input("input/day7.txt").unwrap();

    let now = Instant::now();

    let mut guess: i32 = crabs.iter().sum::<i32>() / crabs.len() as i32;

    let mut lower_bound = *crabs.iter().min().unwrap();
    let mut upper_bound = *crabs.iter().max().unwrap();

    let mut guess_fuel: i32;
    let mut guess_plus_fuel: i32;

    while upper_bound != lower_bound {
        guess_fuel = crabs.iter().map(|v| triangle(i32::abs(v - guess))).sum();
        guess_plus_fuel = crabs
            .iter()
            .map(|v| triangle(i32::abs(v - (guess + 1))))
            .sum();

        if guess_plus_fuel > guess_fuel {
            upper_bound = guess;
            guess = (guess + lower_bound) / 2;
        } else {
            lower_bound = guess + 1;
            guess = (guess + upper_bound) / 2;
        }
    }

    let target = upper_bound;
    let fuel: i32 = crabs.iter().map(|v| triangle(i32::abs(v - target))).sum();

    println!("{}", fuel);
    println!("Finished in {:.5?}", now.elapsed());
}

// Use cached crate or cache by hand?
fn triangle(n: i32) -> i32 {
    n * (n + 1) / 2
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

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_p1(b: &mut Bencher) {
        b.iter(|| {
            solve_p1();
        });
    }

    #[bench]
    fn bench_p2(b: &mut Bencher) {
        b.iter(|| {
            solve_p2();
        });
    }
}
