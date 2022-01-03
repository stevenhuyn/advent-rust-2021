use std::cmp::max;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
pub enum Fold {
    Y(i32),
    X(i32),
}

fn solve_p1() {
    let (mut coords, folds) = read_input("input/day13.txt").unwrap();
    println!("{}", coords.len());

    apply_folds(&mut coords, &folds);

    println!("{}", coords.len());
}

fn apply_folds(coords: &mut HashSet<(i32, i32)>, folds: &Vec<Fold>) {
    for fold in folds {
        for (x, y) in coords.clone() {
            match fold {
                Fold::X(n) => {
                    if x > *n {
                        coords.insert((-1 * x + 2 * n, y));
                        coords.remove(&(x, y));
                    }
                }
                Fold::Y(n) => {
                    if y > *n {
                        coords.insert((x, -1 * y + 2 * n));
                        coords.remove(&(x, y));
                    }
                }
            }
        }
    }
}

fn solve_p2() {
    let (mut coords, folds) = read_input("input/day13.txt").unwrap();
    println!("{}", coords.len());

    apply_folds(&mut coords, &folds);
    println!("{:?}", coords);

    // now to print final
    // get max x and y
    let mut max_x = -1;
    let mut max_y = -1;
    for (x, y) in &coords {
        max_x = max(*x, max_x);
        max_y = max(*y, max_y);
    }

    for y in 0..(max_y + 1) {
        for x in 0..(max_x + 1) {
            if coords.contains(&(x, y)) {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!("");
    }

    println!("{}", coords.len());
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

pub fn read_input(filename: &str) -> Result<(HashSet<(i32, i32)>, Vec<Fold>), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    let mut coords: HashSet<(i32, i32)> = HashSet::new();
    for line_res in lines.by_ref() {
        let line = line_res.unwrap();
        if line == "" {
            break;
        }

        let coord: Vec<i32> = line.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
        coords.insert((coord[0], coord[1]));
    }

    let mut folds: Vec<Fold> = Vec::new();
    for fold_res in lines.by_ref() {
        let fold = fold_res.unwrap();

        let fold_split: Vec<&str> = fold.split("=").collect();
        let fold_num = fold_split[1].parse::<i32>().unwrap();
        if fold_split[0].ends_with("x") {
            folds.push(Fold::X(fold_num))
        } else if fold_split[0].ends_with("y") {
            folds.push(Fold::Y(fold_num))
        } else {
            panic!("HMMM?");
        }
    }

    Ok((coords, folds))
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
