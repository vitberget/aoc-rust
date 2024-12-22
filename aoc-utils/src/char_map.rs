use std::ops::{Add, Sub};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Position {
    pub fn new(x:isize, y:isize) -> Self {
        Self {x, y}
    }
    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

pub type CharMap = HashMap<char, HashSet<Position>>;

pub fn text_to_char_map(text: &str) -> CharMap {
    text.lines().enumerate() 
        .fold(CharMap::new(), |char_map, (y, line)| {
            line.chars().enumerate()
                .fold(char_map, |mut char_map, (x, ch)| {
                    char_map.entry(ch)
                        .and_modify(|set| { set.insert(Position { x: x as isize, y: y as isize}); } )
                        .or_insert(HashSet::from([Position{ x: x as isize, y: y as isize}]));

                    char_map
                })
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_letter() {
        const TEST_TEXT: &str = "a";
        let mut char_map: CharMap = CharMap::new();
        char_map.insert('a', HashSet::from([Position { x: 0, y: 0}]));

        assert_eq!(text_to_char_map(TEST_TEXT), char_map);
    }

    #[test]
    fn test_two_by_two() {
        const TEST_TEXT: &str = "ab\ncd";
        let mut char_map: CharMap = CharMap::new();
        char_map.insert('a', HashSet::from([Position { x: 0, y: 0}]));
        char_map.insert('b', HashSet::from([Position { x: 1, y: 0}]));
        char_map.insert('c', HashSet::from([Position { x: 0, y: 1}]));
        char_map.insert('d', HashSet::from([Position { x: 1, y: 1}]));

        assert_eq!(text_to_char_map(TEST_TEXT), char_map);
    }
}
