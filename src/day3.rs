use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve_p1() {
    let diag_lines = parse_input("input/day3.txt").unwrap();
    let diag_line_length = diag_lines[0].len();
    println!("Line length: {}", diag_line_length);

    let mut count = vec![0; diag_line_length];
    for diag in &diag_lines {
        for i in 0..diag_line_length {
            count[i] += diag[i];
        }
    }

    let mut gamma_vec = vec![0; diag_line_length];
    let mut epsilon_vec = vec![1; diag_line_length];
    for i in 0..diag_line_length {
        if count[i] > (diag_lines.len() as i32) / 2 {
            gamma_vec[i] = 1;
            epsilon_vec[i] = 0;
        }
    }

    let gamma = binary_vec_to_i32(gamma_vec);
    let epsilon = binary_vec_to_i32(epsilon_vec);
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);
    println!("Multiplied: {}", gamma * epsilon)
}

fn binary_vec_to_i32(vec: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut value = 0;
    for n_ptr in vec.iter().rev() {
        let n = *n_ptr;
        if n == 1 {
            value += 2_i32.pow(i);
        }
        i += 1;
    }
    value
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

pub fn parse_input(filename: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();

    let moves = lines
        .map(|line_input| {
            let line = line_input.unwrap();
            let split = line
                .chars()
                .map(|c| c.to_string().parse::<i32>().unwrap())
                .collect();
            split
        })
        .collect();

    Ok(moves)
}
