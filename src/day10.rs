use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_p1() {
    let nav_sys = read_input("input/day10.txt").unwrap();

    let mut _incomplete_lines: Vec<Vec<char>> = Vec::new();
    let mut corrupted_chars: Vec<char> = Vec::new();
    verify_nav(&nav_sys, &mut corrupted_chars, &mut _incomplete_lines);

    let error_score: i32 = corrupted_chars
        .into_iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Unrecognised character"),
        })
        .sum();

    println!("Error score {}", error_score);
}

fn solve_p2() {
    let nav_sys = read_input("input/day10.txt").unwrap();

    let mut _corrupted_chars: Vec<char> = Vec::new();
    let mut incomplete_lines: Vec<Vec<char>> = Vec::new();

    verify_nav(&nav_sys, &mut _corrupted_chars, &mut incomplete_lines);

    let mut scores: Vec<i64> = Vec::new();
    for incomplete_line in incomplete_lines {
        let rev_line = incomplete_line.into_iter().rev();
        let mut line_score: i64 = 0;
        for char in rev_line {
            line_score *= 5;
            line_score += match char {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("Unrecognised character"),
            }
        }
        scores.push(line_score);
    }

    scores.sort();
    let middle_score = scores[scores.len() / 2];
    println!("Scores: {:?}", scores);
    println!("Middle score: {}", middle_score);
}

fn verify_nav(
    nav_sys: &Vec<String>,
    corrupted_chars: &mut Vec<char>,
    incomplete_lines: &mut Vec<Vec<char>>,
) {
    for line in nav_sys {
        let mut stack: Vec<char> = Vec::new();
        let mut is_corrupted = false;
        for char in line.chars() {
            let matched_opt: Option<char> = match char {
                ')' => Some('('),
                ']' => Some('['),
                '>' => Some('<'),
                '}' => Some('{'),
                '(' | '[' | '<' | '{' => None,
                _ => panic!("Unrecognised character"),
            };

            match matched_opt {
                Some(open_char) => {
                    if let Some(&last_char) = stack.last() {
                        if open_char == last_char {
                            stack.pop();
                        } else {
                            corrupted_chars.push(char);
                            is_corrupted = true;
                            break;
                        }
                    }
                }
                None => {
                    stack.push(char);
                }
            }
        }

        // Means incomplte
        if !stack.is_empty() && !is_corrupted {
            incomplete_lines.push(stack.clone());
        }
    }
}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

pub fn read_input(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map(|line| line.unwrap()).collect())
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
