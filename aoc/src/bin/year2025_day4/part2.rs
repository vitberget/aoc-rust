use std::collections::HashSet;

use aoc_procmacros::aoc_profile;
use aoc_utils::position::Position;
use aoc_utils::char_map::CharMap;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let map: CharMap = text.into();

    let starting_paper_rolls = map.get(&'@').unwrap();
    let mut remaining_paper_rolls = starting_paper_rolls.clone();

    loop {
        let remove_these: Vec<Position> = remaining_paper_rolls.clone().into_iter()
            .filter(|position| should_remove(&position, &remaining_paper_rolls))
            .collect();

        if remove_these.is_empty() { break; }

        for pos in remove_these {
            remaining_paper_rolls.remove(&pos);
        }
    }

    Ok(starting_paper_rolls.len() - remaining_paper_rolls.len())
}

fn should_remove(pos: &Position, paper_rolls: &HashSet<Position>) -> bool {
    let surrounding = pos.get_surrounding();
    let intersecting = surrounding.intersection(paper_rolls);
    let count = intersecting.count();
    count < 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 43);
        Ok(())
    }
}
