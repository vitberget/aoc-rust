use aoc_utils::aoc_puzzle;

pub mod part1;

pub fn main() -> anyhow::Result<()> {
    let text = aoc_puzzle!()?;

    // println!("Text:\n{text}");
    println!("Part 1: {}", part1::part_1(&text)?);

    Ok(())
}
