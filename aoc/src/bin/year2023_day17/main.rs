
use parse::text_to_heat_map;

use crate::part_1::solve_part_1;

mod structs;
mod parse;
mod part_1;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    // let filename = get_aoc_filename(args(), 2023, 17);
    // let text = read_to_string(filename)?;
    let text = include_str!("example.txt");

    let heat_map = text_to_heat_map(text)?;

    let p1 = solve_part_1(&heat_map).await?;
    println!("Part 1: {p1}");

    Ok(())
}


