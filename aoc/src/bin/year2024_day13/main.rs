use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;

pub mod parse;
pub mod part_1;
pub mod part_2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 13);
    let text = read_to_string(filename)?;
    let text = text.trim();

    println!("Part 1: {}", part_1::part_1(text));    
    println!("Part 1: {}", part_2::part_2(text));    

    Ok(())
}


