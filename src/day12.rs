use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn solve_p1() {
    let edges = read_input("input/day12.txt").unwrap();
    let adj_map = create_adj_map(edges);

    let mut stack: Vec<(String, Vec<String>)> = Vec::from([("start".to_owned(), Vec::new())]);

    // Inefficient memorywise...?
    let mut path_count = 0;
    while !stack.is_empty() {
        let (u, seen) = stack.pop().unwrap();
        for v in adj_map.get(&u).unwrap() {
            if v == "end" {
                path_count += 1;
                continue;
            }

            if v == "start" {
                continue;
            }

            // Only push it it doesn't contain lowercase v

            if !seen.contains(v) {
                let mut new_seen = seen.clone();
                if !v.chars().next().unwrap().is_uppercase() {
                    // Small cave so add to seen
                    new_seen.push(v.clone());
                }

                stack.push((v.clone(), new_seen));
            }
        }
    }

    println!("{:?}", path_count);
}

fn create_adj_map(edges: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut adj_map: HashMap<String, Vec<String>> = HashMap::new();

    for edge in edges {
        let lhs_edges = adj_map.entry(edge.0.clone()).or_insert(Vec::new());
        if !lhs_edges.contains(&edge.1) {
            lhs_edges.push(edge.1.clone());
        }

        let rhs_edges = adj_map.entry(edge.1).or_insert(Vec::new());
        if !rhs_edges.contains(&edge.0) {
            rhs_edges.push(edge.0);
        }
    }

    adj_map
}

fn solve_p2() {
    let edges = read_input("input/day12.txt").unwrap();
    let adj_map = create_adj_map(edges);

    let mut stack: Vec<(String, Vec<String>, bool)> =
        Vec::from([("start".to_owned(), Vec::new(), false)]);

    // Inefficient memorywise...?
    let mut path_count = 0;
    while !stack.is_empty() {
        let (u, seen, double_cave_visit) = stack.pop().unwrap();

        for v in adj_map.get(&u).unwrap() {
            if v == "end" {
                path_count += 1;
                continue;
            }

            if v == "start" {
                continue;
            }

            // Pushing new cave
            let mut new_seen = seen.clone();
            let mut new_double_cave_visit = double_cave_visit;
            if !v.chars().next().unwrap().is_uppercase() {
                let cave_seen_count = seen.iter().filter(|s| *s == v).count();
                if cave_seen_count == 0 {
                    new_seen.push(v.clone());
                } else if cave_seen_count == 1 && !double_cave_visit {
                    new_double_cave_visit = true;
                    new_seen.push(v.clone());
                } else {
                    continue;
                }
            }
            stack.push((v.clone(), new_seen, new_double_cave_visit));
        }
    }

    println!("{:?}", path_count);
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

pub fn read_input(filename: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    Ok(lines
        .map(|line_res| {
            let line = line_res.unwrap();
            let mut iter = line.split("-");
            (
                iter.next().unwrap().to_owned(),
                iter.next().unwrap().to_owned(),
            )
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
