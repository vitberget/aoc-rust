use std::{collections::HashSet, fmt::{Debug, Write}};

use aoc_utils::char_map::Position;

pub struct Path {
    pub path: Vec<PositionWithDirection>
}

impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Path {")?;
        let mut first = true;
        for position_with_direction in self.path.iter() {
            if first {
                first = false;
            } else {
                f.write_str(", ")?;
            }
            f.write_fmt(format_args!("[{}:{} {} {}]",
                    position_with_direction.position.x,
                    position_with_direction.position.y,
                    match position_with_direction.direction {
                        Direction::North => 'N',
                        Direction::South => 'S',
                        Direction::West => 'W',
                        Direction::East => 'E'
                    },
                    position_with_direction.score
            ))?;
        }
        f.write_str("}")?;
        Ok(())
    }
}

impl Path {
    pub fn is_eating_itself(&self) -> bool {
        let mut path = self.path.iter().rev();
        let mut first = true;
        if let Some(last) = path.next() {
            for next in path {
                if next.same_position(last) && !first {
                    return true;
                }
                first = false;
            }
            false
        } else {
            false
        }
    }


    pub fn is_in_wall(&self, walls: &HashSet<Position>) -> bool {
        let last_pos = self.path.last().unwrap().position;
        walls.contains(&last_pos)
    }


    pub fn is_at_position(&self, position: &Position) -> bool {
        self.path .last()
            .map_or( false, |pos| pos.position == *position)
    }

    pub fn append(&self, position_with_direction: PositionWithDirection) -> Self {
        let mut path = self.path.to_owned();
        path.push(position_with_direction);
        Self { path }
    }

    pub fn next_steps(&self) -> Vec<Path> {
        let last_posi_dir = self.path.last().unwrap();
        vec![
            self.append(last_posi_dir.step()),
            self.append(last_posi_dir.turn_left()),
            self.append(last_posi_dir.turn_right())
        ]
    }
}


#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South
        }
    }
    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North
        }
    }
    pub fn step(&self) -> Position {
        match self {
            Direction::North => Position::new(0, -1),
            Direction::South => Position::new(0, 1),
            Direction::West => Position::new(-1, 0),
            Direction::East => Position::new(1, 0)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PositionWithDirection {
    pub position: Position,
    pub direction: Direction,
    pub score: usize,
}

impl PositionWithDirection {
    pub fn same_position(&self, other: &Self) -> bool { self.position == other.position }
    pub fn same_direction(&self, other: &Self) -> bool { self.direction == other.direction }
    pub fn same_position_and_direction(&self, other: &Self) -> bool {
        self.position == other.position && self.direction == other.direction
    }
    pub fn turn_left(&self) -> Self {
        Self {
            position: self.position,
            direction: self.direction.turn_left(),
            score: self.score + 1000
        }
    }
    pub fn turn_right(&self) -> Self {
        Self {
            position: self.position,
            direction: self.direction.turn_right(),
            score: self.score + 1000
        }
    }
    pub fn step(&self) -> Self {
        Self {
            position: self.position + self.direction.step(),
            direction: self.direction,
            score: self.score + 1
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_not_eating_itself() {
        let path = vec![
            PositionWithDirection { position: Position::new(0, 0), direction: Direction::West, score: 0},
            PositionWithDirection { position: Position::new(0, 0), direction: Direction::West, score: 0},
        ];
        let path = Path { path };
        assert!(! path.is_eating_itself());
    }

    #[test]
    fn test_is_eating_itself() {
        let path = vec![
            PositionWithDirection { position: Position::new(0, 0), direction: Direction::West, score: 0},
            PositionWithDirection { position: Position::new(1, 0), direction: Direction::West, score: 0},
            PositionWithDirection { position: Position::new(0, 0), direction: Direction::West, score: 0},
        ];
        let path = Path { path };
        assert!(path.is_eating_itself());
    }
}
