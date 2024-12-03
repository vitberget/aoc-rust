use std::env::args;
use std::fs::read_to_string;

use aoc_utils::get_aoc_filename;
use parse::text_to_numbers;

use crate::part_1::solve_part_1;
use crate::part_2::solve_part_2;

pub mod parse;
pub mod part_1;
pub mod part_2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 1);
    let text = read_to_string(filename)?;

    let numbers = text_to_numbers(&text)?;

    let p1 = solve_part_1(&numbers);
    println!("Part 1: {p1}");

    let p2 = solve_part_2(&numbers);
    println!("Part 2: {p2}");

    Ok(())
}
