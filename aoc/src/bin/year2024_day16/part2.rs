use std::collections::{HashMap, HashSet};

use aoc_procmacros::aoc_profile;
use aoc_utils::char_map::{text_to_char_map, Position};

use crate::part1::{get_first_path, structs::{Direction, Path}};

#[aoc_profile]
pub fn part_2(text: &str) -> anyhow::Result<usize> {
    let char_map = text_to_char_map(text);
    let walls = char_map.get(&'#').unwrap();
    let end = char_map.get(&'E').unwrap().iter().next().unwrap();

    let mut lowest_score: Option<usize> = None;
    let mut lowest_scored_paths: Vec<Path> = vec![];

    let mut paths: Vec<Path> = vec![get_first_path(&char_map)];

    let mut visited: HashMap<(Position, Direction), usize> = HashMap::new();

    while !paths.is_empty() {
        let new_paths: Vec<Path> = paths.iter()
            .flat_map(|path| path.next_steps())
            .filter(|path| ! path.is_in_wall(walls))
            .filter(|path| ! path.is_eating_itself())
            .collect();

        let ending_paths: Vec<&Path> = new_paths.iter()
            .filter(|path| path.is_at_position(end))
            .collect();

        let lowest_new_end = ending_paths.iter()
            .map(|path| path.path.last().unwrap())
            .map(|item| item.score)
            .min();

        if let Some(current_lowest) = lowest_score {
            if let Some(new_lowest) = lowest_new_end {
                lowest_score = Some(usize::min(current_lowest, new_lowest));
                lowest_scored_paths = vec![];
            }
        } else {
            lowest_score = lowest_new_end;
            lowest_scored_paths = vec![];
        }

        if let Some(current_lowest) = lowest_score {
            for p in ending_paths {
                if p.path.last().unwrap().score == current_lowest {
                    let p: Path = p.clone();
                    lowest_scored_paths.push(p);
                }
            }
        }

        paths = new_paths.into_iter()
            .filter(|path| !path.is_at_position(end))
            .filter(|path| {
                let last = path.path.last().unwrap();
                if let Some(low_score) = visited.get(&(last.position, last.direction)) {
                    last.score <= *low_score
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
    
    let mut sitters: HashSet<Position> = HashSet::new();

    lowest_scored_paths.iter()
        .for_each(|path| path.path.iter()
            .map(|item| item.position)
            .for_each(|pos| { sitters.insert(pos); })
        );

    Ok(sitters.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> anyhow::Result<()> {
        assert_eq!(part_2(include_str!("example_1.txt"))?, 45);
        Ok(())
    }

    #[test]
    fn test_example_2() -> anyhow::Result<()> {
        assert_eq!(part_2(include_str!("example_2.txt"))?, 64);
        Ok(())
    }
}
