use std::collections::HashMap;

use aoc_procmacros::aoc_profile;
use itertools::Itertools;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<String> {
    let the_map = text.trim()
        .lines()
        .fold(HashMap::new(), |mut the_map: HashMap<usize, Vec<char>>, line| {
            line.chars()
                .enumerate()
                .for_each(|(idx, ch)| the_map.entry(idx).or_default().push(ch));
            the_map
        });

    let res: String = the_map.keys()
        .sorted()
        .flat_map(|idx| the_map.get(idx))
        .map(|chars| chars.iter().counts())
        .flat_map(|counts| counts.into_iter().reduce(|a,b| if a.1 > b.1 { a} else { b } ))
        .map(|(ch, _)| ch)
        .collect();

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part1(include_str!("example.txt"))?, "easter");
        Ok(())
    }
}
