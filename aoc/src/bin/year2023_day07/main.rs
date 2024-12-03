use std::env::args;
use std::fs::read_to_string;

use aoc_utils::get_aoc_filename;
use parse::text_to_plays;

use crate::part_1::solve_part_1;
use crate::part_2::solve_part_2;

pub mod structs;
pub mod parse;
pub mod part_1;
pub mod part_2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2023, 7);
    let text = read_to_string(filename)?;
    let plays = text_to_plays(&text)?;

    let p1 = solve_part_1(&plays)?;
    println!("Part 1 {p1}");

    let p2 = solve_part_2(&plays)?;
    println!("Part 2 {p2}");

    Ok(())
}
