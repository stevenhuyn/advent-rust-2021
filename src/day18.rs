use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct RegularNum {
    value: i32,
    depth: i32,
    side: Side,
}

type SnailNum = Vec<RegularNum>;

#[derive(Debug, Copy, Clone, PartialEq)]

enum Side {
    Left,
    Right,
}

fn solve_p1() {
    let snail_lines = read_input("input/test18.txt");

    // println!("Before: {:?}", snail_lines);
    let sol = snail_lines
        .into_iter()
        .reduce(|a, b| reduce(add(a, b)))
        .unwrap();

    println!();

    println!("Solution len {:?}", sol.len());
    println!("Solution: {:?}", sol);

    println!();
    println!("magnitude: {}", magnitude(sol));
}

#[derive(Debug, PartialEq)]
enum SnailState {
    Explode(usize, usize),
    Split(usize),
    Reduced,
}

fn magnitude(mut snail_num: SnailNum) -> i32 {
    while snail_num.len() != 1 {
        // println!("length: {}", snail_num.len());
        // for _ in 0..4 {
        for (i, (left, right)) in snail_num.iter().tuple_windows().enumerate() {
            if left.side == Side::Left && right.side == Side::Right && left.depth == right.depth {
                println!("To Magnitude");
                println!("{:?}", left);
                println!("{:?}", right);

                snail_num[i].depth -= 1;
                snail_num[i].value = snail_num[i].value * 3 + snail_num[i + 1].value * 2;
                snail_num.remove(i + 1);
                snail_num[i].side = get_side(&snail_num, i);
                break;
            }
        }
    }

    snail_num[0].value
}

fn add(left: SnailNum, right: SnailNum) -> SnailNum {
    println!("Add!");
    let mut sum = left;
    sum.extend(right);
    sum.iter_mut().for_each(|reg_num| reg_num.depth += 1);
    sum
}

fn get_side(snail_num: &SnailNum, i: usize) -> Side {
    if (i >= 1
        && snail_num[i - 1].side == Side::Left
        && snail_num[i - 1].depth == snail_num[i].depth)
        || i == snail_num.len() - 1
    {
        return Side::Right;
    }

    if (i < snail_num.len() - 1
        && snail_num[i + 1].side == Side::Right
        && snail_num[i + 1].depth == snail_num[i].depth)
        || i == 0
    {
        return Side::Left;
    }

    println!("{} {:?}", i, snail_num);
    panic!("No Side");
}

fn reduce(mut snail_num: SnailNum) -> SnailNum {
    let mut snail_state = SnailState::Reduced;
    loop {
        // Apply explosion
        if let SnailState::Explode(left_i, right_i) = snail_state {
            if left_i >= 1 {
                snail_num[left_i - 1].value += snail_num[left_i].value;
            }

            if right_i <= snail_num.len() - 2 {
                snail_num[right_i + 1].value += snail_num[right_i].value;
            }

            snail_num.remove(right_i);

            // println!();
            // println!("{:?} {:?}", left_i, snail_num);

            snail_num[left_i].depth -= 1;
            snail_num[left_i].value = 0;
            snail_num[left_i].side = get_side(&snail_num, left_i);

            snail_state = SnailState::Reduced;
        // Apply Split
        } else if let SnailState::Split(i) = snail_state {
            let splitting = snail_num[i];
            let left_value = splitting.value / 2;
            let mut right_value = splitting.value / 2;
            if splitting.value % 2 != 0 {
                right_value += 1;
            }

            snail_num.remove(i);
            snail_num.insert(
                i,
                RegularNum {
                    value: left_value,
                    depth: splitting.depth + 1,
                    side: Side::Left,
                },
            );
            snail_num.insert(
                i + 1,
                RegularNum {
                    value: right_value,
                    depth: splitting.depth + 1,
                    side: Side::Right,
                },
            );
            snail_state = SnailState::Reduced;
        }

        for (i, (left, right)) in snail_num.iter().tuple_windows().enumerate() {
            if left.side == Side::Left
                && right.side == Side::Right
                && left.depth == right.depth
                && left.depth > 4
            {
                // Explode!
                println!("Explode");
                snail_state = SnailState::Explode(i, i + 1);
                break;
            }
        }

        for (i, regular_num) in snail_num.iter().enumerate() {
            if regular_num.value >= 10 {
                // Split!
                println!("Split");
                snail_state = SnailState::Split(i);
                break;
            }
        }

        if snail_state == SnailState::Reduced {
            break;
        }
    }

    snail_num
}

fn solve_p2() {}

pub fn run(day: i32) {
    let now = Instant::now();
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
    println!("Ran in {}", now.elapsed().as_secs_f64());
}

fn read_input(filename: &str) -> Vec<SnailNum> {
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
                        .map(|value| RegularNum {
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
