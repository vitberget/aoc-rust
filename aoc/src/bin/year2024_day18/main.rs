use aoc_utils::aoc_puzzle;

pub mod part1;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;
    let puzzle_text = puzzle_text.trim();

    // let p1 = part1::part1(puzzle_text, 1024, 70, 70));
    println!("Part 1: {}", part1::part1(puzzle_text, 1024, 70, 70)?);

    Ok(())
}
