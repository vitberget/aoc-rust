use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct Position {
    pub(super) x:i64, 
    pub(super) y:i64
}

impl Position {
    pub(super) fn dir(&self, direction: &u8, steps: i64) -> Self {
        let delta = match direction {
            0 => (-1,-1),
            1 => ( 0,-1),
            2 => ( 1,-1),
            3 => (-1, 0),
            4 => ( 1, 0),
            5 => (-1, 1),
            6 => ( 0, 1),
            7 => ( 1, 1),
            _ => ( 0, 0)
        };

        Position {
            x: self.x + delta.0 * steps,
            y: self.y + delta.1 * steps,
        }
    }
}

pub(super) fn text_to_map(text: String) -> anyhow::Result<HashMap<char, HashSet<Position>>> {
    let mut result: HashMap<char, HashSet<Position>> = HashMap::new();
    text.lines().enumerate()
        .for_each(|(y, line)| 
            line.chars().enumerate()
            .for_each(|(x, ch)| {
                let p = Position { x: x as i64, y: y as i64 };
                if let Some(set) = result.get_mut(&ch) {
                    set.insert(p);
                } else {
                    let mut set: HashSet<Position> = HashSet::new();
                    set.insert(p);
                    result.insert(ch, set);
                }
            }));
    Ok(result)
}

pub(super) fn check_letter(position: &Position, map: &HashMap<char, HashSet<Position>>, letter: &char) -> bool {
    map.get(letter)
        .map(|set|set.contains(position))
        .unwrap_or(false)
}
