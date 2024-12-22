use aoc_utils::aoc_puzzle;

mod part1;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;

    println!("Part 1: {}", part1::part_1(&puzzle_text, 100)?);

    Ok(())
}
