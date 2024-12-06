use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;
use common::text_to_map;

mod part1;
mod part2;
mod common;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 4);
    let text = read_to_string(filename)?;
    let map = text_to_map(text)?;

    let part1 = part1::part1(&map);
    println!("Part 1 is {part1}");

    let part2 = part2::part2(&map);
    println!("Part 2 is {part2}");

    Ok(())
}
