use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn solve_p1() {
    let mut grid = read_input("input/day11.txt").unwrap();
    println!("{:?}", grid);

    let xlen = grid.len();
    let ylen = grid[0].len();

    let mut flash_count = 0;

    for _ in 0..100 {
        let mut queue: Vec<(usize, usize)> = Vec::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();

        // Increment everything by 1 then add to queue if a 9
        grid.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, v)| {
                *v += 1;
                if *v > 9 {
                    queue.push((i, j));
                    seen.insert((i, j));
                }
            })
        });

        // Now BFS the frick out of everything
        while !queue.is_empty() {
            let flash_octo = queue.pop().unwrap();
            let (x, y) = flash_octo;

            // get neighbours, flash and push new octopi to 9s in queue
            for (dx, dy) in [-1i32, 0, 1].into_iter().cartesian_product([-1i32, 0, 1]) {
                if x as i32 + dx < 0 || y as i32 + dy < 0 {
                    continue;
                }

                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if (nx, ny) != flash_octo && nx < xlen && ny < ylen {
                    grid[nx][ny] += 1;
                    if !seen.contains(&(nx, ny)) && grid[nx][ny] > 9 {
                        queue.push((nx, ny));
                        seen.insert((nx, ny));
                    }
                }
            }
        }

        // Now convert all flashed octopi to 0
        grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|v| {
                if *v > 9 {
                    *v = 0
                }
            })
        });

        flash_count += seen.len();
    }

    println!("flash count: {}", flash_count);
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

pub fn read_input(filename: &str) -> Result<Vec<Vec<usize>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
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
