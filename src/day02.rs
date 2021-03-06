use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

fn solve_p1() {
    let moves = parse_input("input/day2.txt").unwrap();
    let mut x = 0;
    let mut y = 0;
    for (dx, dy) in moves {
        x += dx;
        y += dy;
    }

    println!("final movement: ({}, {})", x, y);
    println!("Scalar multiple: {}", x * y)
}

fn solve_p2() {
    let moves = parse_input("input/day2.txt").unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (dx, dy) in moves {
        aim += dy;
        x += dx;
        y += aim * dx;
    }

    println!("final movement: ({}, {})", x, y);
    println!("Scalar multiple: {}", x * y)
}

pub fn parse_input(filename: &str) -> io::Result<Vec<(i32, i32)>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let moves = lines
        .map(|line_input| {
            let line = line_input.unwrap();
            let mut split = line.split(" ");
            let direction = split.next().unwrap();
            let magnitude = split.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "forward" => (magnitude, 0),
                "up" => (0, -magnitude),
                "down" => (0, magnitude),
                _ => panic!("bad input"),
            }
        })
        .collect();

    Ok(moves)
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
