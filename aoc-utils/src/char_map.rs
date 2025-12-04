use std::collections::{HashMap, HashSet};

use crate::position::Position;

#[derive(Debug, Clone, PartialEq)]
pub struct CharMap { data: HashMap<char, HashSet<Position>> }

impl CharMap {
    pub fn get(&self, ch: &char) -> Option<&HashSet<Position>> { self.data.get(ch) }

    #[cfg(test)]
    fn new() -> Self { Self { data: HashMap::new() } }
    #[cfg(test)]
    fn insert(&mut self, ch: char, pos: HashSet<Position>) { self.data.insert(ch, pos);}
}

impl From<&str> for CharMap {
    fn from(text: &str) -> Self {
        let data: HashMap<char, HashSet<Position>> = text.lines().enumerate() 
            .fold(HashMap::new(), |char_map, (y, line)| {
                line.chars().enumerate()
                    .fold(char_map, |mut char_map, (x, ch)| {
                        char_map.entry(ch)
                            .and_modify(|set| { set.insert(Position { x: x as isize, y: y as isize}); } )
                            .or_insert(HashSet::from([Position{ x: x as isize, y: y as isize}]));

                        char_map
                    })
            });
        CharMap { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_letter() {
        const TEST_TEXT: &str = "a";
        let mut char_map: CharMap = CharMap::new();
        char_map.insert('a', HashSet::from([Position { x: 0, y: 0}]));
    
        let test_parse: CharMap = TEST_TEXT.into();
        assert_eq!(test_parse, char_map);
    }

    #[test]
    fn test_two_by_two() {
        const TEST_TEXT: &str = "ab\ncd";
        let mut char_map: CharMap = CharMap::new();
        char_map.insert('a', HashSet::from([Position { x: 0, y: 0}]));
        char_map.insert('b', HashSet::from([Position { x: 1, y: 0}]));
        char_map.insert('c', HashSet::from([Position { x: 0, y: 1}]));
        char_map.insert('d', HashSet::from([Position { x: 1, y: 1}]));

        let test_parse: CharMap = TEST_TEXT.into();
        assert_eq!(test_parse, char_map);
    }

    #[test]
    fn test_mul() {
        let test: Position = Position::new(3,4) * 5;
        assert_eq!(test, Position::new(15,20));

        let mut test: Position = Position::new(4,5);
        test *= 5;
        assert_eq!(test, Position::new(20,25));
    }
}
