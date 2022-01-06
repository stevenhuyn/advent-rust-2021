use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn solve_p1() {
    let (xfrom, xto, yfrom, yto) = read_input("input/day17.txt").unwrap();

    let lower_x = inverse_triangle(xfrom).ceil() as usize;
    let upper_x = inverse_triangle(xto).floor() as usize;

    println!("{} {}", inverse_triangle(xfrom), inverse_triangle(xto));

    for x in lower_x..(upper_x + 1) {
        let top_height = triangle(yfrom);

        println!("{} {} {}", x, yfrom, triangle(yfrom.abs() - 1));
        continue;
    }
}

fn solve_p2() {
    let (xfrom, xto, yfrom, yto) = read_input("input/day17.txt").unwrap();

    let mut count = 0;
    for x in 1..xto + 1 {
        for off_y in 0..-yfrom * 2 {
            let y = off_y + yfrom;
            if simulate(xfrom, xto, yfrom, yto, (x, y)) {
                count += 1;
            }
        }
    }

    println!("solutions: {}", count);
}

fn simulate(lower_x: i32, upper_x: i32, lower_y: i32, upper_y: i32, start_vel: (i32, i32)) -> bool {
    let (mut x, mut y) = (0, 0);
    let (mut x_vel, mut y_vel) = start_vel;

    while x <= upper_x && y >= lower_y {
        // println!("x: {}, y: {}", x, y);
        x += x_vel;
        y += y_vel;
        if x >= lower_x && x <= upper_x && y >= lower_y && y <= upper_y {
            return true;
        }
        x_vel = (x_vel - 1).max(0);
        y_vel -= 1;
    }

    false
}

fn inverse_triangle(n: i32) -> f32 {
    (-1f32 + ((1f32 + 8f32 * (n as f32)).sqrt())) / 2f32
}

fn triangle(n: i32) -> i32 {
    (n * (n + 1)) / 2
}

fn displacement(y_vel: i32, steps: i32) -> i32 {
    (y_vel - steps + 1..y_vel + 1).sum()
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

pub fn read_input(filename: &str) -> Result<(i32, i32, i32, i32), Box<dyn Error>> {
    let file = File::open(filename)?;
    let bufreader = BufReader::new(file);
    let mut lines = bufreader.lines();

    let mut line = lines.next().unwrap().unwrap();
    line = line[15..].to_owned();
    let mut split = line.split(", y=");
    let lhs = split.next().unwrap();
    let rhs = split.next().unwrap();

    let mut xvalues = lhs.split("..");
    let mut yvalues = rhs.split("..");

    let xfrom = xvalues.next().unwrap().parse::<i32>().unwrap();
    let xto = xvalues.next().unwrap().parse::<i32>().unwrap();

    let yfrom = yvalues.next().unwrap().parse::<i32>().unwrap();
    let yto = yvalues.next().unwrap().parse::<i32>().unwrap();

    Ok((xfrom, xto, yfrom, yto))
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
