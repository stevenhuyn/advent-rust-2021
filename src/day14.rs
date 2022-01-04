use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::{enumerate, Itertools};

fn solve_p1() {
    solve(10);
}

fn solve_p2() {
    solve(40);
}

fn solve(steps: usize) {
    let (seed, instructions) = read_input("input/day14.txt").unwrap();

    let mut curr: Vec<char> = seed.chars().collect();
    let mut new = curr.clone();
    for _ in 0..steps {
        let mut inserted_count = 0;

        for (i, pair) in enumerate(curr.into_iter().tuple_windows::<(char, char)>()) {
            // Need to use reference iterator because we consume iterator multiple times
            for (pattern, to_insert) in instructions.iter() {
                if pair == *pattern {
                    new.insert(i + 1 + inserted_count, *to_insert);
                    inserted_count += 1;
                    break;
                }
            }
        }
        curr = new.clone();
    }

    let counts: Vec<usize> = curr
        .iter()
        .unique()
        .map(|c| curr.iter().filter(|&v| v == c).count())
        .collect();

    println!("curr len: {}", curr.len());
    println!("counts: {:?}", counts);

    println!(
        "Answer: {}",
        counts.iter().max().unwrap() - counts.iter().min().unwrap()
    );
}

pub fn run(day: i32) {
    let now = Instant::now();
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
    println!("Ran in {}", now.elapsed().as_secs_f64());
}

pub fn read_input(filename: &str) -> Result<(String, Vec<((char, char), char)>), Box<dyn Error>> {
    let file = File::open(filename)?;
    let bufreader = BufReader::new(file);
    let mut lines = bufreader.lines();

    let seed = lines.next().unwrap().unwrap();

    let mut instructions: Vec<((char, char), char)> = Vec::new();
    for line_res in lines.skip(1) {
        let line = line_res.unwrap();
        let mut split = line.split(" -> ");
        let lhs = split.next().unwrap().to_owned();
        let mut lhs_iter = lhs.chars();
        let rhs = split.next().unwrap().to_owned();
        instructions.push((
            (lhs_iter.next().unwrap(), lhs_iter.next().unwrap()),
            rhs.chars().next().unwrap(),
        ));
    }

    Ok((seed, instructions))
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
