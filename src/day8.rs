use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn solve_p1() {
    let lines = read_input("input/day8.txt").unwrap();

    let mut count = 0;
    for line in lines {
        let _uniques = &line[0];
        let output = &line[1];

        for digit in output {
            match digit.len() {
                2 | 3 | 4 | 7 => count += 1,
                _ => (),
            }
        }
    }

    println!("{}", count);
}

/// Brute force cause 7! = 5040 combos
fn solve_p2() {
    let lines = read_input("input/day8.txt").unwrap();
    let now = Instant::now();
    let mut sum = 0;

    for line in (&lines).into_iter() {
        let uniques = &line[0];
        let output = &line[1];

        for perm in String::from("abcdefg").chars().permutations(7) {
            let mut valid_perm = true;
            for digit in uniques.iter().chain(output.iter()) {
                if !verify_digit(&perm, digit) {
                    valid_perm = false;
                    break;
                }
            }

            if valid_perm {
                sum += convert_output(&perm, output);
                break;
            }
        }
    }

    println!("Finished in {:.5?}", now.elapsed());
    println!("{}", sum);
}

const DIG_SOL_ARR: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

/// Verifies a singular permutation for a given digit
fn verify_digit(perm: &Vec<char>, digit: &String) -> bool {
    let sols: HashSet<&str> = HashSet::from(DIG_SOL_ARR);
    let converted_entry: String = remap_digit(perm, digit);
    sols.contains(converted_entry.as_str())
    // DIG_SOL_ARR.contains(&converted_entry.as_str())
}

fn convert_digit(perm: &Vec<char>, digit: &String) -> i32 {
    let converted_entry: String = remap_digit(perm, digit);

    return DIG_SOL_ARR
        .iter()
        .position(|&x| x == converted_entry)
        .unwrap() as i32;
}

fn convert_output(perm: &Vec<char>, output: &Vec<String>) -> i32 {
    let mut num = 0;
    for digit in output {
        let sorted_digit: String = digit.chars().sorted().collect();
        num *= 10;
        num += convert_digit(perm, &sorted_digit);
    }

    num
}

fn remap_digit(perm: &Vec<char>, entry: &String) -> String {
    let res = entry
        .chars()
        .map(|c| match perm.iter().position(|&x| x == c).unwrap() {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            _ => panic!("Bad index"),
        })
        .sorted()
        .collect();

    res
}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

pub fn read_input(filename: &str) -> Result<Vec<Vec<Vec<String>>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let res: Vec<Vec<Vec<String>>> = lines
        .map(|line_res| {
            line_res
                .unwrap()
                .split(" | ")
                .map(|half| half.to_owned())
                .map(|half| half.split(" ").map(|word| word.to_owned()).collect())
                .collect()
        })
        .collect();
    Ok(res)
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
