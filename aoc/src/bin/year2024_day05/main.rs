use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;
use common::SleighLaunchSafetyManual;

mod common;
mod part1;
mod part2;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 5);
    let text = read_to_string(filename)?;
    
    let sleigh_launch_safety_manual: SleighLaunchSafetyManual = text.as_str().into();

    let part1 = part1::part1(&sleigh_launch_safety_manual)?;
    println!("Part 1: {part1}");

    let part2 = part2::part2(&sleigh_launch_safety_manual)?;
    println!("Part 2: {part2}");
    Ok(())
}
