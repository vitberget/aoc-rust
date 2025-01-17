use aoc_utils::aoc_puzzle;

mod part1;
mod part2;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;

    println!("Part 1: {}", part1::part_1(&puzzle_text, 100)?);
    println!("Part 2: {}", part2::part_2(&puzzle_text, 100, 20)?);

    Ok(())
}
