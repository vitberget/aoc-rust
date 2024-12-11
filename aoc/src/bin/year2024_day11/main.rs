use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;

mod part1;
mod part2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 11);
    let text = read_to_string(filename)?;
    let text = text.trim();

    println!("Part 1: {}", part1::blink_many_times(text, 25));
    println!("Part 2: {}", part2::blink_many_times(text, 75));

    Ok(())
}

