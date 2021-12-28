use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve(enable_diag: bool) {
    let mut line_segments = read_input("input/day5.txt").unwrap();

    if !enable_diag {
        line_segments.retain(|v| v[2] - v[0] == 0 || v[3] - v[1] == 0);
    }

    let mut overlaps: HashMap<(i32, i32), i32> = HashMap::new();
    for line_segment in line_segments {
        let mut cur_x = line_segment[0];
        let mut cur_y = line_segment[1];
        *overlaps.entry((cur_x, cur_y)).or_insert(0) += 1;
        while (cur_x, cur_y) != (line_segment[2], line_segment[3]) {
            let delta_x = line_segment[2] - cur_x;
            if delta_x > 0 {
                cur_x += 1
            } else if delta_x < 0 {
                cur_x -= 1;
            }

            let delta_y = line_segment[3] - cur_y;
            if delta_y > 0 {
                cur_y += 1
            } else if delta_y < 0 {
                cur_y -= 1;
            }

            // Inserts if doesn't exist to 0, always increments
            *overlaps.entry((cur_x, cur_y)).or_insert(0) += 1;
        }
    }

    let overlap_count: i32 = overlaps
        .iter()
        .filter_map(|(_k, &v)| if v <= 1 { None } else { Some(1) })
        .sum();

    println!("{:?}", overlap_count);
}

fn solve_p1() {
    solve(false);
}

fn solve_p2() {
    solve(true);
}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

pub fn read_input(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    // Oh god what's the idiomatic way to read input in Rust?
    let values: Vec<Vec<i32>> = lines
        .map(|line| {
            let line_res = line.unwrap();
            let mut res = line_res.split(" -> ");
            let (lhs, rhs) = (res.next().unwrap(), res.next().unwrap());
            let mut lhs_split = lhs.split(",");
            let mut rhs_split = rhs.split(",");
            let (lhsx, lhsy): (i32, i32) = (
                lhs_split.next().unwrap().parse().unwrap(),
                lhs_split.next().unwrap().parse().unwrap(),
            );
            let (rhsx, rhsy): (i32, i32) = (
                rhs_split.next().unwrap().parse().unwrap(),
                rhs_split.next().unwrap().parse().unwrap(),
            );
            vec![lhsx, lhsy, rhsx, rhsy]
        })
        .collect();

    // Ok(lines.map(|a| a.unwrap().parse::<i32>().unwrap()).collect())
    Ok(values)
}
