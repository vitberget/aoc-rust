use std::cmp::Ordering;

use aoc_procmacros::aoc_profile;
use itertools::Itertools;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    Ok(text.trim().lines()
        .filter(|line| is_real_room(line))
        .map(get_sector)
        .sum())
}

pub fn is_real_room(line: &str) -> bool {
    let mut s = line.split("[");
    if let Some(before) = s.next() {
        if let Some(after) = s.next() {
            return is_valid(before, after);
        }   
    }
    false
}

fn is_valid(name: &str, checksum: &str) -> bool {
    let mut char_freqs = name.chars()
        .filter(|ch| ch.is_ascii_lowercase())
        .counts()
        .into_iter()
        .map(|(a, b)| (b, a))
        .collect_vec();

    char_freqs.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1)
        }
    });
    char_freqs.into_iter().take(5)
        .zip(checksum.chars().filter(|ch| ch.is_ascii_lowercase()))
        .all(|(a,b)| { a.1 == b
    })
}

pub fn get_sector(line: &str) -> usize {
    let mut s = line.split("[");
    if let Some(before) = s.next() {
        return before.split("-")
            .last().unwrap_or("0")
            .parse().unwrap_or(0);
    }   
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_real_room() {
        assert!(is_real_room("aaaaa-bbb-z-y-x-123[abxyz]"));
        assert!(is_real_room("a-b-c-d-e-f-g-h-987[abcde]"));
        assert!(is_real_room("not-a-real-room-404[oarel]"));
        assert!(!is_real_room("totally-real-room-200[decoy]"));
    }

    #[test]
    fn test_get_sector() {
        assert_eq!(get_sector("aaaaa-bbb-z-y-x-123[abxyz]"), 123);
        assert_eq!(get_sector("a-b-c-d-e-f-g-h-987[abcde]"), 987);
        assert_eq!(get_sector("not-a-real-room-404[oarel]"), 404);
        assert_eq!(get_sector("totally-real-room-200[decoy]"), 200);
    }

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part1(include_str!("example.txt"))?, 1514);
        Ok(())
    }
}
