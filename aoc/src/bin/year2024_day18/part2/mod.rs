use anyhow::Context;

use crate::part1::part1;

pub fn part2(puzzle_text: &str, no_of_instructions: usize, width: i16, height: i16) -> anyhow::Result<String> {
    let mut no_of_instructions = no_of_instructions;

    while part1(puzzle_text, no_of_instructions, width, height).is_ok() {
        no_of_instructions += 1;
    }
    Ok(puzzle_text.lines().nth(no_of_instructions-1).context("No such line")?.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() -> anyhow::Result<()> {
        assert_eq!(
            part2(include_str!("../example.txt"), 12, 6, 6)?,
            "6,1".to_string()
        );
        Ok(())
    }
}
