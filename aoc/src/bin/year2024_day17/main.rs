use aoc_utils::aoc_puzzle;

pub mod structs;
pub mod parse;
pub mod part1;
pub mod part2;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;

    println!("Part1: {:?}", part1::part1(&puzzle_text)?);
    println!("Part2: {}", part2::part2(&puzzle_text)?);

    Ok(())
}


