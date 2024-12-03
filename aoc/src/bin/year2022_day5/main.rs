use std::env::args;
use std::fs;
use part1::part1;
use crate::part2::part2;

use aoc_utils::get_aoc_filename;

mod part1;
mod part2;
mod parse_text;
mod util;

fn main() {
    let filename = get_aoc_filename(args(), 2022, 5);
    let file_content = fs::read_to_string(filename).unwrap();

    // part 1 TLFGBZHCN
    println!("part1: {}", part1(&file_content));

    // part 2 QRQFHFWCL
    println!("part2: {}", part2(&file_content));
}

