use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
enum SnailNode {
    Value(i32),
    Node(Box<SnailNode>, Box<SnailNode>),
}
#[derive(Debug)]
struct SnailNum {
    root: Option<SnailNode>,
}

#[derive(Debug)]
enum Side {
    Left,
    Right,
}

impl SnailNum {
    fn print(&self) {
        println!("{:?}", &self);
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
            SnailNode::Node(
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

    fn reduce_tree(mut self, depth: i32) {
        if let Some(root) = self.root {
            self.root = Some(SnailNum::reduce_node(root));
        }
    }

    fn reduce_node(node: SnailNode) -> SnailNode {
        if let SnailNode::Value(v) = node {
            if v >= 10 {
                // SPLIT
                let half = v as f32 / 2f32;
                return SnailNode::Node(
                    Box::new(SnailNode::Value(half.ceil() as i32)),
                    Box::new(SnailNode::Value(half.floor() as i32)),
                );
            }
        }
        // } else if let SnailNode::Node(box left, box right) = node {
        // if depth >= 5 {
        //     // EXPLODE
        // }
        // node
        // }

        node
    }

    fn split(node: &mut SnailNode) -> bool {
        if let SnailNode::Value(v) = node {
            if *v >= 5 {
                // SPLIT
                let half = *v as f32 / 2f32;
                *node = SnailNode::Node(
                    Box::new(SnailNode::Value(half.ceil() as i32)),
                    Box::new(SnailNode::Value(half.floor() as i32)),
                );
                return true;
            }

            false
        } else if let SnailNode::Node(left, right) = node {
            SnailNum::split(left) || SnailNum::split(right)
        } else {
            panic!("??")
        }
    }

    fn traverse(node: SnailNode) {
        if let SnailNode::Value(v) = node {
            print!("{} ", v);
        } else if let SnailNode::Node(left, right) = node {
            SnailNum::traverse(*left);
            SnailNum::traverse(*right);
        }
    }

    fn explode(node: SnailNode, depth: usize) -> (Option<i32>, Option<i32>) {
        if let SnailNode::Node(left, right) = node {
            if depth == 4 {
                if let (SnailNode::Value(left_value), SnailNode::Value(right_value)) =
                    (*left, *right)
                {
                    return (Some(left_value), Some(right_value));
                } else {
                    panic!("Depth > 4 but no values")
                }
            }

            // Left child explodes
            let child_exp = SnailNum::explode(*child_node, depth + 1);
            if child_exp != (None, None) {
                return child_exp;
            }

            for child_node in [left, right] {
                let child_exp = SnailNum::explode(*child_node, depth + 1);
                if child_exp != (None, None) {
                    return child_exp;
                }
            }
        }

        return (None, None);
    }

    fn step_add(direction: Side, node: &mut SnailNode, add: i32) {
        if let SnailNode::Node(left, right) = node {
            match direction {
                Side::Left => SnailNum::step_add(direction, left, add),
                Side::Right => SnailNum::step_add(direction, right, add),
            }
        } else if let SnailNode::Value(v) = node {
            *v += add;
        }
    }
}

fn solve_p1() {
    // let snail_nums = read_input("input/test18.txt");

    // for snail_num in snail_nums {
    //     let root = snail_num.root.unwrap();
    //     println!();
    //     SnailNum::traverse(root);
    // }

    let mut snail_num = SnailNum::read_tree("[10,10]");

    snail_num.print();
    let root = snail_num.root.as_mut().unwrap();
    SnailNum::split(root);
    snail_num.print();

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
