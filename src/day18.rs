use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
enum SnailNode {
    Value(i32),
    Tree(Box<SnailNode>, Box<SnailNode>),
}
#[derive(Debug)]
struct SnailNum {
    root: Option<SnailNode>,
}

impl SnailNum {
    fn print(&self) {
        println!("{:?}", self);
    }

    fn read_tree(input: &str) -> SnailNum {
        if input.is_empty() {
            return SnailNum { root: None };
        }

        SnailNum {
            root: Some(SnailNum::read_nodes(input)),
        }
    }

    fn read_nodes(input: &str) -> SnailNode {
        if let Some(mid_comma_ind) = SnailNum::find_middle_comma(input) {
            SnailNode::Tree(
                Box::new(SnailNum::read_nodes(&input[1..mid_comma_ind])),
                Box::new(SnailNum::read_nodes(
                    &input[mid_comma_ind + 1..input.len() - 1],
                )),
            )
        } else {
            SnailNode::Value(input.parse::<i32>().unwrap())
        }
    }

    fn find_middle_comma(input: &str) -> Option<usize> {
        let mut bracket_count = 0;
        for (i, char) in input.chars().enumerate() {
            if char == '[' {
                bracket_count += 1;
            } else if char == ']' {
                bracket_count -= 1;
            } else if char == ',' && bracket_count == 1 {
                return Some(i);
            }
        }

        None
    }
}

fn solve_p1() {
    let snail_nums = read_input("input/test18.txt");

    for snail_num in snail_nums {
        snail_num.print();
    }

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

fn read_input(filename: &str) -> Vec<SnailNum> {
    let file = File::open(filename).unwrap();
    let bufreader = BufReader::new(file);
    let lines = bufreader.lines();

    lines
        .map(|line| SnailNum::read_tree(&line.unwrap()))
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
