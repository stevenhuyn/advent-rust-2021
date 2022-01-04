use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::{enumerate, Itertools};

fn solve_p1() {
    let (seed, instructions) = read_input("input/day14.txt").unwrap();

    let mut curr: Vec<char> = seed.chars().collect();
    let mut new = curr.clone();
    for _ in 0..10 {
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

fn solve_p2() {
    let (seed, instructions) = read_input("input/day14.txt").unwrap();

    let mut pair_counter: HashMap<(char, char), i64> = HashMap::new();
    let mut unique_chars: HashSet<char> = HashSet::new();

    for char in seed.chars() {
        unique_chars.insert(char);
    }

    for pair in seed.chars().into_iter().tuple_windows::<(char, char)>() {
        *pair_counter.entry(pair).or_insert(0) += 1;
    }

    for _ in 0..40 {
        let mut counter_delta: HashMap<(char, char), i64> = HashMap::new();
        for (&pair, &amount) in pair_counter.iter() {
            for &(pattern, to_insert) in instructions.iter() {
                if pair == pattern {
                    unique_chars.insert(to_insert);
                    *counter_delta.entry(pair).or_insert(0) -= amount;
                    *counter_delta.entry((pair.0, to_insert)).or_insert(0) += amount;
                    *counter_delta.entry((to_insert, pair.1)).or_insert(0) += amount;
                    break;
                }
            }
        }

        for (pair, delta) in counter_delta {
            let entry = pair_counter.entry(pair).or_insert(0);
            *entry += delta;
            if *entry == 0 {
                pair_counter.remove(&pair);
            }
        }
    }

    let mut counts: Vec<i64> = Vec::new();
    for char in unique_chars {
        let mut l_count = 0;
        let mut r_count = 0;
        for ((l, r), amount) in pair_counter.iter() {
            if *l == char {
                l_count += amount;
            }

            if *r == char {
                r_count += amount;
            }
        }

        if seed.chars().next().unwrap() == char {
            l_count += 1;
        }

        if seed.chars().last().unwrap() == char {
            r_count += 1;
        }

        counts.push((l_count + r_count) / 2);
    }

    println!("counts {:?}", counts);
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
