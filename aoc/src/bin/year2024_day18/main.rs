use aoc_utils::aoc_puzzle;

pub mod part1;
pub mod part2;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;
    let puzzle_text = puzzle_text.trim();

    println!("Part 1: {}", part1::part1(puzzle_text, 1024, 70, 70, true)?);
    println!("Part 2: {:?}", part2::part2(puzzle_text, 1024, 70, 70)?);

    Ok(())
}
