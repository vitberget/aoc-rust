use aoc_procmacros::aoc_profile;
use aoc_utils::char_map::text_to_char_map;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let map = text_to_char_map(text);
    let paper_rolls = map.get(&'@').unwrap();

    let available = paper_rolls.iter()
        .map(|paper_roll| paper_roll.get_surrounding())
        .map(|surrounding| surrounding.intersection(paper_rolls).count())
        .filter(|number_of_surrounding| *number_of_surrounding < 4)
        .count();

    Ok(available)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part1(EXAMPLE)?;
        assert_eq!(result, 13);
        Ok(())
    }
}
