use std::collections::{HashMap, HashSet};

use aoc_procmacros::aoc_profile;
use aoc_utils::char_map::CharMap;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let map: CharMap = text.into();

    let mut beams: HashMap<isize, usize> = map.get(&'S').unwrap()
        .iter()
        .map(|p| (p.x, 1))
        .collect();

    let splitters = map.get(&'^').unwrap();
    let max_y = map.get_max_y();

    for y in 1..max_y {
        let splitter_xs: HashSet<isize> = splitters.iter()
            .filter(|p| p.y == y)
            .map(|p| p.x)
            .collect();

        let beams_xs: HashSet<isize> = beams.keys().copied().collect();

        for x in beams_xs {
            if splitter_xs.contains(&x) {
                let count = *beams.get(&x).unwrap_or(&0);
                beams.remove(&x);

                let new_x = x - 1;
                match beams.get_mut(&new_x) {
                    Some(values) => { *values += count; }
                    None => { beams.insert(new_x, count); }
                }

                let new_x = x + 1;
                match beams.get_mut(&new_x) {
                    Some(values) => { *values += count; }
                    None => { beams.insert(new_x, count); }
                }
            }
        }
    }

    Ok(beams.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 40);
        Ok(())
    }
}
