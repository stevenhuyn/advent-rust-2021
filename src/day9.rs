use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_p1() {
    let map = read_input("input/day9.txt").unwrap();

    let lowpoints = find_basins(&map);
    let mut risk_level = 0;

    for (x, y) in lowpoints {
        risk_level += map[x][y] + 1;
    }

    println!("risk level: {}", risk_level);
}

fn solve_p2() {
    println!("p2 answer")
}

fn find_basins(map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut lowpoints = Vec::new();
    let xlen = map.len() as i32;
    let ylen = map[0].len() as i32;
    for x in 0..xlen {
        for y in 0..ylen {
            let mut is_lowpoint = true;
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                if x + dx >= 0 && x + dx < xlen && y + dy >= 0 && y + dy < ylen {
                    if map[(x + dx) as usize][(y + dy) as usize] <= map[x as usize][y as usize] {
                        is_lowpoint = false;
                        break;
                    }
                }
            }

            if is_lowpoint {
                lowpoints.push((x as usize, y as usize));
            }
        }
    }

    lowpoints
}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

pub fn read_input(filename: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines
        .map(|a| {
            a.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect())
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
