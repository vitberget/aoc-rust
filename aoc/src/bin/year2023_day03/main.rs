use std::env::args;
use std::fs::read_to_string;

use aoc_utils::get_aoc_filename;
use parse::parse_text_to_schematic;

use crate::part_1::solve_part_1;
use crate::part_2::solve_part_2;

mod structs;
mod parse;
mod part_1;
mod part_2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2023, 3);
    let text = read_to_string(filename)?;

    let schematic = parse_text_to_schematic(&text)?;

    let p1 = solve_part_1(&schematic)?;
    println!("Part 1: {p1}");

    let p2 = solve_part_2(&schematic)?;
    println!("Part 2: {p2}");

    Ok(())
}
