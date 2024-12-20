use std::collections::HashSet;
use std::str::FromStr;

use anyhow::{bail, Context};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct MemPos {
    pub x: i16,
    pub y: i16
}

impl FromStr for MemPos {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut split = text.split(",");
        let x = split.next().context("No x")?;
        let y = split.next().context("No y")?;

        let x: i16 = x.parse()?;
        let y: i16 = y.parse()?;

        Ok(Self { x, y })
    }
}

#[derive(Debug)]
pub struct Memory {
    pub width: i16, 
    pub height: i16,

    pub start_pos: MemPos,
    pub end_pos: MemPos,

    pub corrupted: HashSet<MemPos>
}

pub fn part1(puzzle_text: &str, no_of_instructions: usize, width: i16, height: i16) -> anyhow::Result<usize> {
    let memory = text_to_memory(puzzle_text, no_of_instructions, width, height);

    let mut visited: HashSet<MemPos> = HashSet::from([memory.start_pos]);
    let mut count = 0; 
    let mut current_positions: HashSet<MemPos> = HashSet::from([memory.start_pos]);

    while ! current_positions.is_empty() {
        current_positions = update_paths(&current_positions, &visited, &memory);
        count += 1;

        if current_positions.iter().any(|mem_pos| *mem_pos == memory.end_pos) {
            return Ok(count);
        }

        current_positions.iter().for_each(|mem_pos| { visited.insert(*mem_pos); });
    }

    bail!("current positions got empty") 
}

pub fn update_paths(current_positions: &HashSet<MemPos>, visited: &HashSet<MemPos>, memory: &Memory) -> HashSet<MemPos> {
        current_positions.iter()
            .flat_map(|mem_pos| vec![
                MemPos { x: mem_pos.x + 1 , y: mem_pos.y },
                MemPos { x: mem_pos.x - 1, y: mem_pos.y },
                MemPos { x: mem_pos.x, y: mem_pos.y + 1},
                MemPos { x: mem_pos.x, y: mem_pos.y - 1},
            ])
            .filter(|mem_pos| ! visited.contains(mem_pos))
            .filter(|mem_pos| ! memory.corrupted.contains(mem_pos))
            .filter(|mem_pos| mem_pos.x >= 0 
                && mem_pos.x <= memory.width 
                && mem_pos.y >= 0 
                && mem_pos.y <= memory.height )
            .collect()
}

pub fn text_to_memory(puzzle_text: &str, no_of_instructions: usize, width: i16, height: i16) -> Memory {
    let corrupted: HashSet<MemPos> = puzzle_text.lines()
        .take(no_of_instructions)
        .flat_map(|line| line.parse::<MemPos>())
        .collect();

    let start_pos = MemPos { x: 0, y: 0 };
    let end_pos = MemPos { x: width, y: height };

    Memory { corrupted, start_pos, end_pos, width, height }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() -> anyhow::Result<()> {
        assert_eq!(
            part1(include_str!("../example.txt"), 12, 6, 6)?,
            22
        );
        Ok(())
    }
}
