use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn solve_p1() {
    let (seed, instructions) = read_input("input/test14.txt").unwrap();

    println!("{:?}, {:?}", seed, instructions);
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

pub fn read_input(filename: &str) -> Result<(String, Vec<(String, String)>), Box<dyn Error>> {
    let file = File::open(filename)?;
    let bufreader = BufReader::new(file);
    let mut lines = bufreader.lines();

    let seed = lines.next().unwrap().unwrap();

    let mut instructions: Vec<(String, String)> = Vec::new();
    for line_res in lines.skip(1) {
        let line = line_res.unwrap();
        let mut split = line.split(" -> ");
        let lhs = split.next().unwrap().to_owned();
        let rhs = split.next().unwrap().to_owned();
        instructions.push((lhs, rhs));
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
