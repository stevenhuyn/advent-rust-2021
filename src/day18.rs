use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
struct SnailNum {
    value: i32,
    depth: i32,
    side: Side,
}

#[derive(Debug, Copy, Clone)]

enum Side {
    Left,
    Right,
}

fn solve_p1() {
    let snail_lines = read_input("input/test18.txt");
    println!("{:?}", snail_lines);
    println!("p1 answer")
}

fn solve_p2() {
    println!("p2 answer")
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

fn read_input(filename: &str) -> Vec<Vec<SnailNum>> {
    let file = File::open(filename).unwrap();
    let bufreader = BufReader::new(file);
    let lines = bufreader.lines();

    // Absoluty disgusting
    lines
        .map(|line| {
            let mut depth = 0;
            let mut side = Side::Left;
            line.unwrap()
                .chars()
                .filter_map(|c| {
                    c.to_digit(10)
                        .or_else(|| {
                            match c {
                                '[' => {
                                    depth += 1;
                                    side = Side::Left
                                }
                                ']' => depth -= 1,
                                ',' => side = Side::Right,
                                _ => (),
                            }
                            None
                        })
                        .map(|value| SnailNum {
                            value: value as i32,
                            depth,
                            side,
                        })
                })
                .collect()
        })
        .collect()
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
