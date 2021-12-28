use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve_p1() {
    let (draws, mut games) = read_input("input/day4.txt").unwrap();

    for draw in draws {
        for game in games.iter_mut() {
            mark(draw, game);
            if check_win(game) {
                println!("Solution: {}", solve(game) * draw);
                return;
            }
        }
    }
}

fn solve_p2() {
    let (draws, mut games) = read_input("input/day4.txt").unwrap();

    let mut losers: Vec<usize> = (0..games.len()).collect();
    let mut last_game_flag = false;
    for draw in draws {
        for (i, game) in games.iter_mut().enumerate() {
            mark(draw, game);

            if check_win(game) {
                losers.retain(|&game_index| game_index != i);
                if losers.len() == 1 {
                    last_game_flag = true;
                    continue;
                }

                if last_game_flag {
                    println!("Solution: {}", solve(&game) * draw);
                    return;
                }
            }
        }
    }
}

fn mark(draw: i32, game: &mut Vec<Vec<i32>>) {
    let board_width = game.len();

    for i in 0..board_width {
        for j in 0..board_width {
            if game[i][j] == draw {
                game[i][j] = -1;
            }
        }
    }
}

fn check_win(game: &Vec<Vec<i32>>) -> bool {
    let board_width = game.len();

    for row in 0..board_width {
        if (0..board_width).all(|col| game[row][col] == -1) {
            return true;
        }
    }

    for col in 0..board_width {
        if (0..board_width).all(|row| game[row][col] == -1) {
            return true;
        }
    }

    false
}

fn solve(game: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in game {
        for val in row {
            sum += if *val == -1 { 0 } else { *val }
        }
    }

    sum
}

pub fn run(day: i32) {
    match day {
        1 => solve_p1(),
        2 => solve_p2(),
        _ => println!("Unknown part!"),
    }
}

pub fn read_input(filename: &str) -> io::Result<(Vec<i32>, Vec<Vec<Vec<i32>>>)> {
    let file = File::open(filename)?;
    let mut lines = BufReader::new(file).lines();

    let draws: Vec<i32> = lines
        .next()
        .unwrap()?
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let mut games: Vec<Vec<Vec<i32>>> = Vec::new();
    for line in lines {
        let line_str = line.unwrap();
        if line_str == "" {
            games.push(Vec::new());
            continue;
        }

        let row = line_str
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        games.last_mut().unwrap().push(row);
    }

    Ok((draws, games))
}
