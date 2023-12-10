use advent_of_code_2023::file_io::lines_from_file;
use itertools::Itertools;
use std::{collections::HashMap, env, path::Path};
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let file_path = Path::new("inputs/day3/input.txt");
    day1(file_path);
    let file_path = Path::new("inputs/day3/input.txt");
    day2(file_path);
}

fn day1(file_path: impl AsRef<Path>) {
    const SEP: char = '.';

    let lines = lines_from_file(file_path);
    let height = lines.len();
    let width = lines[0].len();
    let input: String = lines.join("");

    let mut current_num = "".to_string();
    let mut included = false;
    let mut total = 0;
    for (i, c) in input.char_indices() {
        if c.is_numeric() {
            let adj = adj_idx(&i, &width, &height);

            for adj_idx in adj {
                let adj_c = input.as_bytes()[adj_idx] as char;
                if !adj_c.is_numeric() && (adj_c != SEP) {
                    included = true;
                    break;
                }
            }
            current_num.push(c)
        } else {
            if included {
                match current_num.parse::<i32>() {
                    Ok(num) => {
                        total = total + num;
                    }
                    Err(_) => (),
                }
            }
            current_num = "".to_string();
            included = false;
        }
    }
    println!("Part 1: {:?}", total);
}

fn day2(file_path: impl AsRef<Path>) {
    const GEAR: char = '*';

    let lines = lines_from_file(file_path);
    let height = lines.len();
    let width = lines[0].len();
    let input: String = lines.join("");

    let invalid_index = input.len();
    let mut number_map: HashMap<usize, i32> = HashMap::new();
    let mut gear_index: usize = invalid_index;
    let mut current_num = "".to_string();
    let mut total = 0;
    for (i, c) in input.char_indices() {
        if c.is_numeric() {
            let adj = adj_idx(&i, &width, &height);

            for adj_idx in adj {
                let adj_c = input.as_bytes()[adj_idx] as char;
                if adj_c == GEAR {
                    gear_index = adj_idx;
                    break;
                }
            }
            current_num.push(c)
        } else if current_num.len() > 0 {
            let num = match current_num.parse::<i32>() {
                Ok(num) => num,
                Err(_) => panic!("Couldn't convert {current_num} to i32"),
            };

            match number_map.get(&gear_index) {
                Some(n1) => {
                    total = total + (n1 * num);
                }
                None => {
                    if gear_index != invalid_index {
                        number_map.insert(gear_index, num);
                    }
                }
            }
            current_num = "".to_string();
            gear_index = invalid_index;
        }
    }
    println!("Part 2: {:?}", total);
}

fn adj_idx(idx: &usize, width: &usize, height: &usize) -> Vec<usize> {
    let idx = *idx as i32;
    let width = *width as i32;
    let height = *height as i32;

    let x = idx % width;
    let y = idx / height;

    let mut adj: Vec<usize> = Vec::new();
    for (xo, yo) in (-1..2).cartesian_product(-1..2) {
        let xp = x + xo;
        let yp = y + yo;
        if (xp >= 0) && (xp < width) && (yp >= 0) && (yp < height) {
            adj.push((yp * width + xp) as usize);
        }
    }

    return adj;
}
