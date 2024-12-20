use std::time::Duration;
use std::{collections::HashSet, thread::sleep};
use std::str::FromStr;

use anyhow::{bail, Context};
use aoc_utils::color;

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

pub fn part1(puzzle_text: &str, no_of_instructions: usize, width: i16, height: i16, print: bool) -> anyhow::Result<usize> {
    let memory = text_to_memory(puzzle_text, no_of_instructions, width, height);

    let mut visited: HashSet<MemPos> = HashSet::from([memory.start_pos]);
    let mut count = 0; 
    let mut current_positions: HashSet<MemPos> = HashSet::from([memory.start_pos]);

    if print { print_it(&memory, &visited, &current_positions)}

    while ! current_positions.is_empty() {
        current_positions = update_paths(&current_positions, &visited, &memory);
        count += 1;

        if current_positions.iter().any(|mem_pos| *mem_pos == memory.end_pos) {
            return Ok(count);
        }

        current_positions.iter().for_each(|mem_pos| { visited.insert(*mem_pos); });
        if print { print_it(&memory, &visited, &current_positions)}
    }

    bail!("current positions got empty") 
}

fn print_it(memory: &Memory, visited: &HashSet<MemPos>, current_positions: &HashSet<MemPos>) {
    println!();
    let mut text: String = String::new();
    text.push_str(color::RED);
    text.push_str(&color::goto(0,0));
    text.push('\n');

    for y in 0..=memory.height {
        for x in 0..=memory.width {
            let mp = MemPos { x, y };
            if mp == memory.start_pos {
                text.push_str(color::YELLOW);
                text.push('S');
            } else if mp == memory.end_pos {
                text.push_str(color::YELLOW);
                text.push('E');
            } else if current_positions.contains(&mp) {
                text.push_str(color::WHITE);
                text.push('O');
            } else if visited.contains(&mp) {
                text.push_str(color::LIGHT_GRAY);
                text.push('O');
            } else if memory.corrupted.contains(&mp) {
                text.push_str(color::LIGHT_RED);
                text.push('#');
            } else {
                text.push(' ');
            }
        }
        text.push('\n');
    }
    text.push_str(color::RESET);
    println!("{text}");
    sleep(Duration::from_millis(100));
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
            part1(include_str!("../example.txt"), 12, 6, 6, false)?,
            22
        );
        Ok(())
    }
}
