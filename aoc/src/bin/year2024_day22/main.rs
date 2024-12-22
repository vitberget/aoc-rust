use aoc_utils::aoc_puzzle;

use crate::part1::solve_part_1;
use crate::part2::solve_part_2;

pub mod part1;
pub mod part2;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;

    println!("Part1: {}", solve_part_1(&puzzle_text)?);
    println!("Part2: {}", solve_part_2(&puzzle_text)?);

    Ok(())
}

