use std::ops::RangeInclusive;

use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let mut ranges = parse(text)?;

    while let Some((range_1, range_2, merged_range)) = get_a_merger(&ranges) {
        ranges.remove(range_2);
        ranges.remove(range_1);
        ranges.push(merged_range);
    } 

    Ok(ranges.iter()
        .map(|range| 1 + range.end() - range.start())
        .sum())
}

fn parse(text: &str) -> anyhow::Result<Vec<RangeInclusive<usize>>> {
    let mut ranges: Vec<RangeInclusive<usize>> = vec![];

    for line in text.lines() {
        if line.contains("-") && let Some((start, end)) = line.split_once('-') {
            let start:usize = start.parse()?;
            let end:usize = end.parse()?;

            ranges.push(start..=end);
        } else {
            break;
        }
    }

    Ok(ranges)
}

fn get_a_merger(ranges: &[RangeInclusive<usize>]) -> Option<(usize,usize,RangeInclusive<usize>)> {
    for range_1 in 0..(ranges.len()-1) {
        for range_2 in (range_1+1)..ranges.len() {
            if let Some(merged_range) = merge_if_intersecting(&ranges[range_1], &ranges[range_2]) {
                return Some((range_1, range_2, merged_range))
            }
        }
    } 
    None
}

fn merge_if_intersecting(range_1: &RangeInclusive<usize>, range_2: &RangeInclusive<usize>) -> Option<RangeInclusive<usize>> {
    if *range_1.end() >= range_2.start() - 1 && range_1. start() - 1 <= *range_2.end() 
        || *range_2.end() >= range_1.start() - 1 && range_2.start() - 1 <= *range_1.end() {
            let start = usize::min(*range_1.start(), *range_2.start());
            let end = usize::max(*range_1.end(), *range_2.end());

            Some(start..=end)
        } else {
            None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_1a() {
        let range_1 = 1..=10;
        let range_2 = 29..=40;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, None);
    }

    #[test]
    fn merge_1b() {
        let range_2 = 1..=10;
        let range_1 = 29..=40;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, None);
    }

    fn merge_1c() {
        let range_1 = 1..=10;
        let range_2 = 12..=40;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, None);
    }

    #[test]
    fn merge_1d() {
        let range_2 = 1..=10;
        let range_1 = 12..=40;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, None);
    }

    #[test]
    fn merge_2a() {
        let range_1 = 1..=10;
        let range_2 = 10..=12;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=12));
    }

    #[test]
    fn merge_2b() {
        let range_2 = 1..=10;
        let range_1 = 10..=12;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=12));
    }

    #[test]
    fn merge_3a() {
        let range_1 = 1..=9;
        let range_2 = 10..=12;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=12));
    }

    #[test]
    fn merge_3b() {
        let range_2 = 1..=9;
        let range_1 = 10..=12;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=12));
    }

    #[test]
    fn merge_4a() {
        let range_1 = 1..=9;
        let range_2 = 7..=12;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=12));
    }

    #[test]
    fn merge_4b() {
        let range_2 = 1..=9;
        let range_1 = 7..=12;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=12));
    }

    #[test]
    fn merge_5a() {
        let range_1 = 1..=9;
        let range_2 = 7..=8;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=9));
    }

    #[test]
    fn merge_5b() {
        let range_2 = 1..=9;
        let range_1 = 7..=8;

        let result = merge_if_intersecting(&range_1, &range_2);

        assert_eq!(result, Some(1..=9));
    }

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 14);
        Ok(())
    }
}
