use std::collections::HashSet;
use std::sync::LazyLock;

use aoc_procmacros::aoc_profile;
use aoc_utils::position::Position;
use regex::Regex;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let mut the_map: HashSet<Position> = HashSet::new();

    text.trim().lines()
        .for_each(|line| match line {
            line if line.starts_with("rect ") => {
                if let Some(rect) = get_rect(line) {
                    for x in 0..rect.0 {
                        for y in 0..rect.1 {
                            the_map.insert(Position::new(x, y));
                        }
                    }
                }
                 
            }
            line if line.starts_with("rotate column ") => {
                if let Some(rotate) = get_rotate(line) {
                    let positions: Vec<Position> = the_map.iter()
                        .filter(|pos| pos.x == rotate.0)
                        .map(|pos| pos.to_owned())
                        .collect();

                    positions.iter().for_each(|pos| { the_map.remove(pos); });

                    positions.iter()
                        .map(|pos| Position { x: pos.x, y: (pos.y + rotate.1) % 6})
                        .for_each(|pos| { the_map.insert(pos); });
                };
            }
            line if line.starts_with("rotate row ") => {
                if let Some(rotate) = get_rotate(line) {
                    let positions: Vec<Position> = the_map.iter()
                        .filter(|pos| pos.y == rotate.0)
                        .map(|pos| pos.to_owned())
                        .collect();

                    positions.iter().for_each(|pos| { the_map.remove(pos); });

                    positions.iter()
                        .map(|pos| Position { x: (pos.x + rotate.1) % 50, y: pos.y})
                        .for_each(|pos| { the_map.insert(pos); });
                };
            }

            _ => {}
        });
    Ok(the_map.len())
}
pub fn get_rect(line: &str) -> Option<(isize, isize)> {
    static RECT_REGEX: LazyLock<Regex> = LazyLock::new(||
            Regex::new(r"(?<left>\d+)x(?<right>\d+)").expect("Bad regex")
        );

    if let Some(captures) = RECT_REGEX.captures(line) {
        let left:isize = captures["left"].parse().unwrap();
        let right:isize = captures["right"].parse().unwrap();
        return Some((left, right));
    }

    None
}

pub fn get_rotate(line: &str) -> Option<(isize, isize)> {
    static ROTATE_REGEX: LazyLock<Regex> = LazyLock::new(||
            Regex::new(r"(?<left>\d+) by (?<right>\d+)").expect("Bad regex")
        );

    if let Some(captures) = ROTATE_REGEX.captures(line) {
        let left:isize = captures["left"].parse().unwrap();
        let right:isize = captures["right"].parse().unwrap();
        return Some((left, right));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rotate() {
       assert_eq!(get_rotate("rotate column x=1 by 1"), Some((1, 1))); 
       assert_eq!(get_rotate("rotate row y=0 by 4"), Some((0, 4))); 
    }
    #[test]
    fn test_get_rect() {
       assert_eq!(get_rect("rect 3x2"), Some((3, 2))); 
    }
}
