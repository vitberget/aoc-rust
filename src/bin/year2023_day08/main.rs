use std::env::args;
use std::fs::read_to_string;

use aoc_utils::get_aoc_filename;
use parse::text_to_desert_map;

use crate::part_1::solve_part_1;
use crate::part_2::solve_part_2;

pub mod structs;
pub mod parse;
pub mod part_1;
pub mod part_2;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2023, 8);
    let text = read_to_string(filename)?;
    let (instructions, network) = text_to_desert_map(&text)?;

    let p1 = solve_part_1(&instructions, &network)?;
    println!("Part 1: {p1}");

    let p2 = solve_part_2(&instructions, &network).await?;
    println!("Part 2: {p2}");

    Ok(())
}


