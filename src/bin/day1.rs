use phf::phf_map;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

static DIGIT_WORDS: phf::Map<&str, char> = phf_map! {
    "one"=>'1',
    "two"=>'2',
    "three"=>'3',
    "four"=>'4',
    "five"=>'5',
    "six"=>'6',
    "seven"=>'7',
    "eight"=>'8',
    "nine"=>'9',
};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = Path::new("inputs/day1/part1_input.txt");
    day1(file_path);
    let file_path = Path::new("inputs/day1/part2_input.txt");
    day2(file_path);
}

fn day1(file_path: impl AsRef<Path>) {
    let lines = lines_from_file(file_path);
    let mut total = 0;
    for line in lines {
        let mut result: String = "".to_owned();
        for char in line.chars() {
            if char.is_digit(10) {
                result.push(char);
                break;
            }
        }
        for char in line.chars().rev() {
            if char.is_digit(10) {
                result.push(char);
                break;
            }
        }
        let int_result = result.parse::<i32>().unwrap();
        total += int_result;
    }
    println!("Part 1: {}", total);
}

fn day2(file_path: impl AsRef<Path>) {
    let lines = lines_from_file(file_path);
    let mut total = 0;
    for line in lines {
        let mut result: String = "".to_owned();
        match process_forward(&line) {
            None => (),
            Some(c) => result.push(c),
        }
        match process_backward(&line) {
            None => (),
            Some(c) => result.push(c),
        }
        let int_result = result.parse::<i32>().unwrap();
        total += int_result;
    }
    println!("Part 2: {}", total);
}

fn process_forward(line: &String) -> Option<char> {
    let mut this_word: String = "".to_owned();
    for char in line.chars() {
        if char.is_digit(10) {
            return Some(char);
        } else {
            this_word.push(char)
        }
        let check_str: &str = &this_word;
        for key in DIGIT_WORDS.keys() {
            if check_str.contains(key) {
                match DIGIT_WORDS.get(key) {
                    None => (),
                    Some(c) => return Some(*c),
                }
            }
        }
    }
    None
}

fn process_backward(line: &String) -> Option<char> {
    let mut this_word: String = "".to_owned();
    for char in line.chars().rev() {
        if char.is_digit(10) {
            return Some(char);
        } else {
            this_word.push(char)
        }
        let check_str: &str = &this_word.chars().rev().collect::<String>();
        for key in DIGIT_WORDS.keys() {
            if check_str.contains(key) {
                match DIGIT_WORDS.get(key) {
                    None => (),
                    Some(c) => return Some(*c),
                }
            }
        }
    }
    None
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
