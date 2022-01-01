use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn solve_p1() {
    let edges = read_input("input/test12.txt").unwrap();
    let adj_map = create_adj_map(edges);

    let mut stack: Vec<String> = Vec::from(["start".to_owned()]);
    let mut seen_small: HashMap<String, Vec<String>> =
        HashMap::from([("start".to_owned(), Vec::new())]);

    // Inefficient memorywise...?
    let mut path_count = 0;
    // while !stack.is_empty() {
    for _ in 0.. {
        let u = stack.pop().unwrap();

        let u_trav = &seen_small.get(&u).unwrap().clone();

        for v in adj_map.get(&u).unwrap() {
            println!("{:?}", seen_small);
            if v == "end" {
                path_count += 1;
                continue;
            }

            if v == "start" {
                continue;
            }

            if !u_trav.contains(v) {
                stack.push(v.clone());
                if let None = seen_small.get(v) {
                    seen_small.insert(v.clone(), Vec::new());
                }

                if !v.chars().next().unwrap().is_uppercase() && v != "end" {
                    let u_trav = seen_small.get_mut(&u).unwrap().clone();
                    let v_trav = seen_small.get_mut(v).unwrap();
                    v_trav.extend([v.clone()]);
                    v_trav.extend(u_trav);
                }
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

fn solve_p2() {}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
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
