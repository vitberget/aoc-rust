use std::collections::HashSet;

use aoc_procmacros::aoc_profile;
use aoc_utils::char_map::CharMap;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let map: CharMap = text.into();
    
    let mut beams: HashSet<isize> = map.get(&'S').unwrap()
        .iter()
        .map(|p| p.x)
        .collect();

    let splitters = map.get(&'^').unwrap();
    let max_y = map.get_max_y();

    let mut number_of_splits = 0;
    for y in 1..max_y {
        let splitter_xs: HashSet<isize> = splitters.iter()
            .filter(|p| p.y == y)
            .map(|p| p.x)
            .collect();

        let hits: HashSet<isize> = splitter_xs.intersection(&beams).copied().collect();

        number_of_splits += hits.len();

        beams = beams.difference(&hits).copied().collect();

        let new_beams: HashSet<isize> = hits.into_iter()
            .flat_map(|x| vec![x-1,x+1])
            .collect();

        beams = beams.union(&new_beams).copied().collect();
    }

    

    Ok(number_of_splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part1(EXAMPLE)?;
        assert_eq!(result, 21);
        Ok(())
    }
}
