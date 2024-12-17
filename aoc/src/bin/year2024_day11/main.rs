use aoc_utils::aoc_puzzle;

mod part1;
mod part2;

pub fn main() -> anyhow::Result<()> {
    let text = aoc_puzzle!()?;
    let text = text.trim();

    println!("Part 1: {}", part1::blink_many_times(text, 25));
    println!("Part 2: {}", part2::blink_many_times(text, 75));

    Ok(())
}
