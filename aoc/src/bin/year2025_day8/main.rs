use aoc_utils::aoc_puzzle;

pub mod part1;
pub mod part2;
pub mod structs;

pub fn main() -> anyhow::Result<()> {
    let text = aoc_puzzle!()?;

    println!("Part 1: {}", part1::part1(&text, 1000)?);
    println!("Part 2: {}", part2::part2(&text)?);

    Ok(())
}
