use itertools::izip;
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
    let depths = read_input("input/day1.txt").unwrap();

    let mut count = 0;
    for window in depths.windows(2) {
        if let [a, b] = window {
            if b > a {
                count += 1
            }
        }
    }
    println!("{}", count)
}

fn solve_p2() {
    let depths = read_input("input/day1.txt").unwrap();

    let mut count = 0;
    for (a, b) in izip!(depths.windows(3), depths[1..].windows(3)) {
        if b.iter().sum::<i32>() > a.iter().sum() {
            count += 1;
        }
    }
    println!("{}", count)
}

pub fn read_input(filename: &str) -> io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines.map(|a| a.unwrap().parse::<i32>().unwrap()).collect())
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
