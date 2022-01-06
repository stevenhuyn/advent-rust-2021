use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn solve_p1() {
    let hex_packet = read_input("input/day16.txt").unwrap();
    let bits = packet_to_bits(&hex_packet);
    println!("{} {:?}", hex_packet, bits);

    let (version_sum, _) = recurse_packet(&bits);

    println!("version sum: {}", version_sum);
}

fn solve_p2() {
    println!("p2 answer")
}

fn recurse_packet(packet: &str) -> (usize, usize) {
    let version = usize::from_str_radix(&packet[..3], 2).unwrap();
    let type_id = usize::from_str_radix(&packet[3..6], 2).unwrap();

    println!("Recurse! ver: {}, type: {}", version, type_id);
    let mut index_parsed = 6usize;

    let mut version_sum: usize = version;

    if type_id == 4 {
        // Literal value
        let (literal_string, index_offset) = build_literal(&packet[6..], 0);
        index_parsed += index_offset;
        let literal = usize::from_str_radix(&literal_string, 2).unwrap();
        println!("literal: {}, {}", literal, index_parsed);
    } else {
        // Operator packet
        let length_type_id = &packet[6..7];
        if length_type_id == "0" {
            // next 15 bits are total length of packets
            let length = usize::from_str_radix(&packet[7..22], 2).unwrap();

            // Read twice
            index_parsed = 22;
            while index_parsed < 22 + length {
                recurse_packet_helper(&mut version_sum, &mut index_parsed, packet);
            }
        } else if length_type_id == "1" {
            // Number of sub packets
            let sub_packets = usize::from_str_radix(&packet[7..18], 2).unwrap();
            index_parsed = 18;
            for _ in 0..sub_packets {
                recurse_packet_helper(&mut version_sum, &mut index_parsed, packet);
            }
        }
    }

    (version_sum, index_parsed)
}

fn recurse_packet_helper(version_sum: &mut usize, index_parsed: &mut usize, packet: &str) {
    let (rest_version_sum, index_offset) = recurse_packet(&packet[*index_parsed..]);
    *version_sum += rest_version_sum;
    *index_parsed += index_offset;
}

fn build_literal(literal: &str, index_parsed: usize) -> (String, usize) {
    let mut answer = "".to_owned();
    answer.push_str(&literal[1..5]);

    let mut new_index_parsed = 5;
    if literal[0..1] == *"1" {
        let (rest_str, index_offset) = build_literal(&literal[5..], new_index_parsed);
        new_index_parsed += index_offset;
        answer.push_str(&rest_str);
    }

    println!("str: {}, parsed: {}", literal, new_index_parsed);
    (answer, new_index_parsed)
}

fn packet_to_bits(hex_packet: &str) -> String {
    let mut binary_string = "".to_owned();
    for char in hex_packet.chars() {
        binary_string.push_str(match char {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        })
    }

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
