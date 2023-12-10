use advent_of_code_2023::file_io::lines_from_file;
use regex::Regex;
use std::{env, iter::zip, path::Path};
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = Path::new("inputs/day2/input.txt");
    day1(file_path);
    let file_path = Path::new("inputs/day2/input.txt");
    day2(file_path);
}

fn day1(file_path: impl AsRef<Path>) {
    const RED_MAX: i32 = 12;
    const GREEN_MAX: i32 = 13;
    const BLUE_MAX: i32 = 14;

    let lines = lines_from_file(file_path);
    let id_pattern = Regex::new(r"^Game (\d+)").unwrap();
    let red_pattern = Regex::new(r"(\d+) red").unwrap();
    let green_pattern = Regex::new(r"(\d+) green").unwrap();
    let blue_pattern = Regex::new(r"(\d+) blue").unwrap();

    let patterns: [&Regex; 3] = [&red_pattern, &green_pattern, &blue_pattern];
    let maximums: [i32; 3] = [RED_MAX, GREEN_MAX, BLUE_MAX];

    let mut possible_games: Vec<i32> = Vec::new();

    for line in lines {
        let mut allowed = true;
        for (pattern, max) in zip(patterns, maximums) {
            let captures = capture_vector(pattern, &line);
            let maxval = captures.iter().max().expect("Vector is empty");
            if maxval > &max {
                allowed = false;
                break;
            }
        }
        if allowed {
            let Some(game_id) = id_pattern.captures(&line) else {
                println!("Could not find game id");
                return;
            };
            let game_id = game_id[1].parse::<i32>().unwrap();
            possible_games.push(game_id);
        }
    }
    let solution: i32 = possible_games.iter().sum();
    println!("Part 1: {:?}", solution);
}

fn day2(file_path: impl AsRef<Path>) {
    let lines = lines_from_file(file_path);
    let red_pattern = Regex::new(r"(\d+) red").unwrap();
    let green_pattern = Regex::new(r"(\d+) green").unwrap();
    let blue_pattern = Regex::new(r"(\d+) blue").unwrap();

    let patterns: [&Regex; 3] = [&red_pattern, &green_pattern, &blue_pattern];

    let mut solution: i32 = 0;

    for line in lines {
        let mut min_amounts: Vec<i32> = Vec::new();
        for pattern in patterns {
            let captures = capture_vector(pattern, &line);
            let maxval = captures.iter().max().expect("Vector is empty");
            min_amounts.push(*maxval)
        }
        solution = solution + min_amounts.iter().product::<i32>();
    }
    println!("Part 2: {:?}", solution);
}

fn capture_vector(pattern: &Regex, input: &String) -> Vec<i32> {
    let counts: Vec<i32> = pattern
        .captures_iter(input)
        .map(|caps| {
            let (_, [count]) = caps.extract();
            let count = count.parse::<i32>().unwrap();
            count
        })
        .collect();
    return counts;
}
