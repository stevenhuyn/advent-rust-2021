use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve_p1() {
    let coords = read_input("input/test5.txt").unwrap();
    println!("{:?}", coords);
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

pub fn read_input(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

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
