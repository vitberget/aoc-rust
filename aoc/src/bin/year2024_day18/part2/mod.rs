use std::collections::HashSet;

use anyhow::Context;

use crate::part1::part1;
use crate::part1::print::print_it;

pub fn part2(puzzle_text: &str, no_of_instructions: usize, width: i16, height: i16, print: bool) -> anyhow::Result<String> {
    let mut no_of_instructions = no_of_instructions;

    while let Ok((_, memory)) = part1(puzzle_text, no_of_instructions, width, height, false, 0) {
        no_of_instructions += 1;
        if print { print_it(&memory, &HashSet::new(), &HashSet::new(), 10);}
    }
    Ok(puzzle_text.lines().nth(no_of_instructions-1).context("No such line")?.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() -> anyhow::Result<()> {
        assert_eq!(
            part2(include_str!("../example.txt"), 12, 6, 6, false)?,
            "6,1".to_string()
        );
        Ok(())
    }
}
