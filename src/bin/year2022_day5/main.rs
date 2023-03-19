use std::fs;
use part1::part1;
use crate::part2::part2;

mod part1;
mod part2;
mod parse_text;

fn main() {
    let file_content = fs::read_to_string("puzzles/year2022_day5.txt").unwrap();

    // part 1 TLFGBZHCN
    println!("part1: {}", part1(&file_content));

    // part 2 QRQFHFWCL
    println!("part2: {}", part2(&file_content));
}

