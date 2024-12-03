use std::{env::args, fs::read_to_string};

use aoc_utils::get_aoc_filename;

use crate::part_1::solve_part_1;
use crate::part_2::solve_part_2;

mod part_1;
mod part_2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2023, 15);
    let text = read_to_string(filename)?;
    let text = text.trim();

    let p1 = solve_part_1(text)?;
    println!("Part 1: {p1}");

    let p2 = solve_part_2(text)?;
    println!("Part 2: {p2}");

    Ok(())
}

