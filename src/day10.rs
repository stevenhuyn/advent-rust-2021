use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_p1() {
    let nav_sys = read_input("input/day10.txt").unwrap();

    let mut stack: Vec<char> = Vec::new();
    let mut corrupted_chars: Vec<char> = Vec::new();
    for line in nav_sys {
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
                            break;
                        }
                    }
                }
                None => {
                    stack.push(char);
                }
            }
        }
    }

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
    println!("p2 answer")
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
