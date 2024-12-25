use std::collections::HashMap;

use aoc_utils::char_map::{text_to_char_map, CharMap, Position};
use structs::{Direction, Path, PositionWithDirection};

pub mod structs;

pub fn part_1(text: &str) -> anyhow::Result<usize> {
    let char_map = text_to_char_map(text);
    let walls = char_map.get(&'#').unwrap();
    let end = char_map.get(&'E').unwrap().iter().next().unwrap();

    let mut lowest_score: Option<usize> = None;
    let mut paths: Vec<Path> = vec![get_first_path(&char_map)];

    let mut visited: HashMap<(Position, Direction), usize> = HashMap::new();

    while !paths.is_empty() {
        let new_paths: Vec<Path> = paths.iter()
            .flat_map(|path| path.next_steps())
            .filter(|path| ! path.is_in_wall(walls))
            .filter(|path| ! path.is_eating_itself())
            .collect();

        let lowest_new_end = new_paths.iter()
            .filter(|path| path.is_at_position(end))
            .map(|path| path.path.last().unwrap())
            .map(|item| item.score)
            .min();

        if let Some(current_lowest) = lowest_score {
            if let Some(new_lowest) = lowest_new_end {
                lowest_score = Some(usize::min(current_lowest, new_lowest));
            }
        } else {
            lowest_score = lowest_new_end;
        }

        paths = new_paths.into_iter()
            .filter(|path| !path.is_at_position(end))
            .filter(|path| {
                let last = path.path.last().unwrap();
                if let Some(low_score) = visited.get(&(last.position, last.direction)) {
                    last.score < *low_score
                } else {
                    true
                }

            }).filter(|path| if let Some(lowest_score) = lowest_score {
                path.path.last().unwrap().score < lowest_score
            } else {
                true
            }).collect();

        paths.iter()
            .map(|path| path.path.last().unwrap())
            .for_each(|item| { visited.insert((item.position, item.direction), item.score); });
    }

    Ok(lowest_score.unwrap_or(0))
}

pub fn get_first_path(char_map: &CharMap) -> Path {
    let pos_with_dir = PositionWithDirection {
        position: *char_map.get(&'S').unwrap().iter().next().unwrap(),
        direction: structs::Direction::East,
        score: 0
    };

    let path = vec![pos_with_dir];

    Path { path }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> anyhow::Result<()> {
        assert_eq!(part_1(include_str!("../example_1.txt"))?, 7036);
        Ok(())
    }

    #[test]
    fn test_example_2() -> anyhow::Result<()> {
        assert_eq!(part_1(include_str!("../example_2.txt"))?, 11048);
        Ok(())
    }
}
