use aoc_utils::aoc_puzzle;

pub mod part1;
pub mod part2;

pub fn main() -> anyhow::Result<()> {
    let puzzle_text = aoc_puzzle!()?;
    let puzzle_text = puzzle_text.trim();

    let print_it = std::env::var("AOC_PRINTING").is_ok();

    println!("Part 1: {}", part1::part1(puzzle_text, 1024, 70, 70, print_it, 100)?.0);
    println!("Part 2: {:?}", part2::part2(puzzle_text, 1024, 70, 70, print_it)?);

    Ok(())
}
