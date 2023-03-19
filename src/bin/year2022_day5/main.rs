use std::env::args;
use std::fs;
use part1::part1;
use crate::part2::part2;

mod part1;
mod part2;
mod parse_text;
mod util;

fn main() {
    let filename = get_filename();
    let file_content = fs::read_to_string(filename).unwrap();

    // part 1 TLFGBZHCN
    println!("part1: {}", part1(&file_content));

    // part 2 QRQFHFWCL
    println!("part2: {}", part2(&file_content));
}

/// Returns first argument as filename, or a default
fn get_filename() -> String {
    let filename = "puzzles/year2022_day5.txt".to_string();
    return args()
        .into_iter()
        .nth(1)
        .or_else(|| Some(filename))
        .unwrap();
}

