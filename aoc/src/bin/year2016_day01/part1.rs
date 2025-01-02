use aoc_procmacros::aoc_profile;

pub enum Direction {
    North,
    South,
    West,
    East
}

pub struct Position { 
    pub direction: Direction, 
    pub x: isize, 
    pub y: isize
}

impl Position {
    pub fn right(&mut self, number: isize) {
        match self.direction {
            Direction::North => {
                self.direction = Direction::East;
                self.x += number;
            }
            Direction::South => {
                self.direction = Direction::West;
                self.x -= number;
            }
            Direction::West => {
                self.direction = Direction::North;
                self.y -= number;
            }
            Direction::East => {
                self.direction = Direction::South;
                self.y += number;
            }
        }
    }
    pub fn left(&mut self, number: isize) {
        match self.direction {
            Direction::South => {
                self.direction = Direction::East;
                self.x += number;
            }
            Direction::North => {
                self.direction = Direction::West;
                self.x -= number;
            }
            Direction::East => {
                self.direction = Direction::North;
                self.y -= number;
            }
            Direction::West => {
                self.direction = Direction::South;
                self.y += number;
            }
        }
    }
}

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<isize> {
    let final_position: Position = text.trim().split(", ")
        .fold(Position { direction: Direction::North, x: 0, y: 0 },
            |mut pos, word|{
                if let Ok(number) = word[1..].parse::<isize>() {
                    match word.chars().next() {
                        Some('L') => pos.left(number),
                        Some('R') => pos.right(number),

                        _ => {}
                    }
                }

                pos
            }
        );
    Ok(final_position.x.abs() + final_position.y.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> anyhow::Result<()> {
        assert_eq!(part1("R2, L3")?, 5); 
        Ok(())
    }

    #[test]
    fn test_example_2() -> anyhow::Result<()> {
        assert_eq!(part1("R2, R2, R2")?, 2); 
        Ok(())
    }

    #[test]
    fn test_example_3() -> anyhow::Result<()> {
        assert_eq!(part1("R5, L5, R5, R3")?, 12); 
        Ok(())
    }
}
