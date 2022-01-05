use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn solve_p1() {
    let hex_packet = read_input("input/test16.txt").unwrap();
    let bits = packet_to_bits(&hex_packet);
    println!("{} {:?}", hex_packet, bits);

    recurse_packet(&bits);
}

fn solve_p2() {
    println!("p2 answer")
}

fn recurse_packet(packet: &str) {
    let version = usize::from_str_radix(&packet[..3], 2).unwrap();
    let type_id = usize::from_str_radix(&packet[3..6], 2).unwrap();

    match type_id {
        4 => {
            // Literal value
            let literal_string = build_literal(&packet[6..]);
            let literal = usize::from_str_radix(&literal_string, 2).unwrap();
            println!("literal: {}", literal);
        }
        _ => {}
    }

    println!("version: {}, type: {}", version, type_id);
}

fn build_literal(literal: &str) -> String {
    let mut answer = "".to_owned();
    answer.push_str(&literal[1..5]);

    if literal[0..1] == *"1" {
        answer.push_str(&build_literal(&literal[5..]));
    }

    answer
}

fn packet_to_bits(hex_packet: &str) -> String {
    let value = u32::from_str_radix(hex_packet, 16).unwrap();
    let binary_string = format!("{:b}", value);

    binary_string
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

pub fn read_input(filename: &str) -> Result<String, Box<dyn Error>> {
    let file = File::open(filename)?;
    let bufreader = BufReader::new(file);
    let mut lines = bufreader.lines();

    Ok(lines.next().unwrap().unwrap())
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
