use std::collections::{HashMap, HashSet};

use crate::common::{check_letter, Position};

pub(super) fn part2(map: &HashMap<char, HashSet<Position>>) -> usize {
    map.get(&'A').unwrap().iter()
        .filter(|pos| is_part_of_x_mas(true, pos, map) && is_part_of_x_mas(false, pos, map))
        .count()
}

fn is_part_of_x_mas(cross_direction: bool, pos: &Position, map: &HashMap<char, HashSet<Position>>) -> bool {
    let (c1, c2) = if cross_direction { 
        (pos.dir(&0, 1), pos.dir(&7, 1)) 
    } else { 
        (pos.dir(&2, 1), pos.dir(&5, 1)) 
    };

    check_letter(&c1, map, &'M') && check_letter(&c2, map, &'S')
    || check_letter(&c1, map, &'S') && check_letter(&c2, map, &'M')
}

