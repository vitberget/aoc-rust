use std::collections::HashSet;

use anyhow::bail;
use aoc_procmacros::aoc_profile;

use crate::part1::{Direction, Position};

impl Position {
    pub fn turn_right(&mut self, number: isize) -> (isize, isize) {
         match self.direction {
            Direction::North => {
                self.direction = Direction::East;
                self.x += number;
                (1, 0)
            }
            Direction::South => {
                self.direction = Direction::West;
                self.x -= number;
                (-1, 0)
            }
            Direction::West => {
                self.direction = Direction::North;
                self.y -= number;
                (0, -1)
            }
            Direction::East => {
                self.direction = Direction::South;
                self.y += number;
                (0, 1)
            }
        }
    }
    pub fn turn_left(&mut self, number: isize) -> (isize, isize) {
        match self.direction {
            Direction::South => {
                self.direction = Direction::East;
                self.x += number;
                (1, 0)
            }
            Direction::North => {
                self.direction = Direction::West;
                self.x -= number;
                (-1, 0)
            }
            Direction::East => {
                self.direction = Direction::North;
                self.y -= number;
                (0, -1)
            }
            Direction::West => {
                self.direction = Direction::South;
                self.y += number;
                (0, 1)
            }
        }
    }
}

#[aoc_profile]
#[allow(unreachable_code)]
pub fn part2(text: &str) -> anyhow::Result<isize> {
    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    visited.insert((0,0));
    let mut position = Position { direction: crate::part1::Direction::North, x: 0, y:0 };

    for word in text.trim().split(", ") {
        let mut px = position.x;
        let mut py = position.y;

        let number: isize = word[1..].parse()?;
        let (dx,dy) = match word.chars().next()  {
            Some('L') => position.turn_left(number),
            Some('R') => position.turn_right(number),

            _ => bail!("WTF")
        };

        for _ in 0..number {
            px += dx;
            py += dy;

            if !visited.insert((px, py)) {
                return Ok(px.abs() + py.abs())
            }
        }
    }

    bail!("Exited the entire thing!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part2("R8, R4, R4, R8")?, 4);
        Ok(())
    }
}
