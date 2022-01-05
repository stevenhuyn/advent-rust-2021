use std::collections::{BinaryHeap, HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use itertools::enumerate;

fn solve_p1() {
    let grid = read_input("input/day15.txt").unwrap();

    // Generate graph
    let mut adj_map: HashMap<(usize, usize), Vec<((usize, usize), i32)>> = HashMap::new();
    let mut tentative: HashMap<(usize, usize), i32> = HashMap::new();
    gen_graph(&grid, &mut adj_map, &mut tentative);

    // Run dijkstras
    let mut heap: BinaryHeap<(i32, (usize, usize))> = BinaryHeap::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    heap.push((0, (0, 0)));

    let mut stepped_once = false;
    while !heap.is_empty() {
        let (_, u) = heap.pop().unwrap();
        let u_dist = *tentative.get(&u).unwrap();

        if stepped_once {
            seen.insert(u);
        }
        stepped_once = true;

        if u == (grid.len() - 1, grid[0].len() - 1) {
            break;
        }

        for (v, uv_dist) in adj_map.get(&u).unwrap() {
            let uv_tent = tentative.get_mut(v).unwrap();
            if u_dist + uv_dist < *uv_tent && !seen.contains(v) {
                *uv_tent = u_dist + uv_dist;
                heap.push((-(*uv_tent), *v));
            }
        }
    }

    // Get bottom right tentative distance (the min distance)
    println!(
        "Min risk: {:?}",
        tentative.get(&(grid.len() - 1, grid[0].len() - 1))
    );
}

fn gen_graph(
    grid: &Vec<Vec<i32>>,
    adj_map: &mut HashMap<(usize, usize), Vec<((usize, usize), i32)>>,
    tentative: &mut HashMap<(usize, usize), i32>,
) {
    let xlen = grid.len() as i32;
    let ylen = grid[0].len() as i32;
    for (i, row) in enumerate(grid.iter()) {
        for (j, _) in enumerate(row) {
            tentative.insert((i, j), i32::MAX);

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let newx = i as i32 + dx;
                let newy = j as i32 + dy;
                if newx < 0 || newx >= xlen || newy < 0 || newy >= ylen {
                    continue;
                }

                let adjacents = adj_map.entry((i, j)).or_insert(Vec::new());
                adjacents.push((
                    (newx as usize, newy as usize),
                    grid[newx as usize][newy as usize],
                ));
            }
        }
    }
    tentative.insert((0, 0), 0);
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

pub fn read_input(filename: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let bufreader = BufReader::new(file);
    let lines = bufreader.lines();

    Ok(lines
        .map(|line_res| {
            line_res
                .unwrap()
                .chars()
                .map(|char| char.to_digit(10).unwrap() as i32)
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
