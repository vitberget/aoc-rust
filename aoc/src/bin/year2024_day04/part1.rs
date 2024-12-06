use std::collections::{HashMap, HashSet};

use crate::common::{check_letter, Position};

pub(super) fn part1(map: &HashMap<char, HashSet<Position>>) -> usize {
    map.get(&'X').unwrap().iter()
        .map(|pos| count_xmas(pos, map))
        .sum()
}

fn count_xmas(pos: &Position, map: &HashMap<char, HashSet<Position>>) -> usize {
    (0..=7)
        .filter(|direction| is_xmas(pos, direction, map))
        .count()
}

fn is_xmas(pos: &Position, direction: &u8, map: &HashMap<char, HashSet<Position>>) -> bool {
    check_letter(&pos.dir(direction, 1), map, &'M')
    && check_letter(&pos.dir(direction, 2), map, &'A')
    && check_letter(&pos.dir(direction, 3), map, &'S')
}
