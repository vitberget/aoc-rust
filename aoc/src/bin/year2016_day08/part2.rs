use std::collections::HashSet;

use aoc_procmacros::aoc_profile;
use aoc_utils::char_map::Position;

use crate::part1::{get_rect, get_rotate};

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
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

    for y in 0..6 {
        for x in 0..50 {
            let pos = Position::new(x, y);
            if the_map.contains(&pos) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }

    Ok(0)
}
