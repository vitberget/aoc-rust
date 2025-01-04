pub mod part1;
pub mod part2;

pub fn main() -> anyhow::Result<()> {
    println!("Part 1: {}", part1::part1("ffykfhsq")?);
    println!("Part 2: {}", part2::part2("ffykfhsq")?);

    Ok(())
}
