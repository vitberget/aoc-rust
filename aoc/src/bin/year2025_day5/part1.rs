use std::ops::RangeInclusive;

use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let mut ranges: Vec<RangeInclusive<usize>> = vec![];
    let mut ingridients: Vec<usize> = vec![];

    for line in text.lines() {
        if line.contains("-") {
            if let Some((start, end)) = line.split_once('-') {
                let start:usize = start.parse()?;
                let end:usize = end.parse()?;

                ranges.push(start..=end);
            }
        } else if ! line.is_empty() {
            ingridients.push(line.parse()?);
        }
    }

    Ok(ingridients.iter()
        .filter(|id| contains_ingridient(id, &ranges))
        .count())
}

fn contains_ingridient(ingridient: &usize, ranges: &[RangeInclusive<usize>]) -> bool {
    ranges.iter().any(|range| range.contains(ingridient)) 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part1(EXAMPLE)?;
        assert_eq!(result, 3);
        Ok(())
    }
}
