use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve_p1() {
    let lines = read_input("input/day8.txt").unwrap();

    let mut count = 0;
    for line in lines {
        let _uniques = &line[0];
        let output = &line[1];

        for digit in output {
            count += match digit.len() {
                2 => 1,
                3 => 1,
                4 => 1,
                7 => 1,
                _ => 0,
            }
        }
    }

    println!("{}", count);
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

pub fn read_input(filename: &str) -> io::Result<Vec<Vec<Vec<String>>>> {
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
