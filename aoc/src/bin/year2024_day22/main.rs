use aoc_utils::aoc_puzzle;
use part1::solve_part_1;

pub mod part1;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;

    println!("Part1: {}", solve_part_1(&puzzle_text)?);
    Ok(())
}

