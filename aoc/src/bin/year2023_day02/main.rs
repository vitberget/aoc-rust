use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;
use parse::parse_to_games;

use crate::part_1::part_1;
use crate::part_2::part_2;

mod structs;
mod parse;
mod part_1;
mod part_2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2023, 2);
    let text = read_to_string(filename)?;

    let games = parse_to_games(text)?;

    let p1: u32 = part_1(&games)?;
    println!("Part 1: {p1}");

    let p2: u32 = part_2(&games)?;
    println!("Part 2: {p2}");

    Ok(())
}
