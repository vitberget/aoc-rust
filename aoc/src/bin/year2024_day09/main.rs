use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;
use common::text_to_diskmap;

mod common;
mod part1;
mod part2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 9);
    let text = read_to_string(filename)?;
    let text = text.trim();


    println!("Part 1: {}", part1::part1(text_to_diskmap(text))?);

    println!("Part 2: {}", part2::part2(text_to_diskmap(text))?);
    // 6 307 279 963 620

    Ok(())
}

