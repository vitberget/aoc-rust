use std::env::args;
use std::fs::read_to_string;

use aoc_utils::get_aoc_filename;
use parse::parse;

use crate::part_1::solve_part_1;
use crate::part_2::solve_part_2;

pub mod structs;
pub mod parse;
pub mod part_1;
pub mod part_2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2023, 4);
    let text = read_to_string(filename)?;
    let cards = parse(&text)?;

    let p1 = solve_part_1(&cards)?;
    println!("Part 1: {p1}");
    
    let p2 = solve_part_2(&cards)?;
    println!("Part 2: {p2}");

    Ok(())
}

