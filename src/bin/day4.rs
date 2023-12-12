use advent_of_code_2023::file_io::lines_from_file;
use itertools::enumerate;
use regex::Regex;
use std::{collections::HashMap, collections::HashSet, env, path::Path};
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = Path::new("inputs/day4/input.txt");
    day1(file_path);
    let file_path = Path::new("inputs/day4/input.txt");
    day2(file_path);
}

fn day1(file_path: impl AsRef<Path>) {
    let lines = lines_from_file(file_path);
    let pattern =
        Regex::new(r"^Card\s+\d+:\s+(?<winning>(?:\d*\s+)+)\|(?<actual>(?:\d*\s*)+)$").unwrap();
    let mut score = 0;
    for line in lines {
        let Some(ranges) = pattern.captures(&line) else {
            panic!("Failed to match {:?}", line)
        };
        let winning: HashSet<u32> = ranges["winning"]
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let actual: HashSet<u32> = ranges["actual"]
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let n_wins: u32 = winning.intersection(&actual).count() as u32;
        if n_wins > 0 {
            score = score + 2_usize.pow(n_wins - 1);
        }
    }
    println!("Part 1: {:?}", score);
}

fn day2(file_path: impl AsRef<Path>) {
    let lines = lines_from_file(file_path);
    let pattern =
        Regex::new(r"^Card\s+\d+:\s+(?<winning>(?:\d*\s+)+)\|(?<actual>(?:\d*\s*)+)$").unwrap();
    let mut counts: HashMap<_, _> = (1..lines.len() + 1)
        .into_iter()
        .map(|i| (i, 1_u32))
        .collect();
    for (i, line) in enumerate(lines) {
        let Some(ranges) = pattern.captures(&line) else {
            panic!("Failed to match {:?}", line)
        };
        let winning: HashSet<u32> = ranges["winning"]
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        let actual: HashSet<u32> = ranges["actual"]
            .trim()
            .split_ascii_whitespace()
            .map(|num| num.parse::<u32>().unwrap())
            .collect();

        let n_wins = winning.intersection(&actual).count();
        let this_game = i + 1;
        let this_tally = *counts.get(&this_game).unwrap_or(&1);
        for j in this_game + 1..this_game + n_wins + 1 {
            let tally = counts.entry(j).or_insert(1);
            *tally += this_tally;
        }
    }
    let score: u32 = counts.values().sum();
    println!("Part 2: {:?}", score);
}
