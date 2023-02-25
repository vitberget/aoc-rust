use std::collections::HashSet;
use std::str::Lines;
use std::time::Instant;

use crate::part1::score_letter;

pub(crate) fn part2() {
    let now = Instant::now();
    // Include file in binary
    let score = solve_part2(include_str!("../../../puzzles/year2022-day3.txt").lines());
    let dur = now.elapsed();

    println!("Part 2 year2022-day3.txt = {} in {:?}", score, dur);
}

fn solve_part2(mut lines: Lines) -> u32 {
    let mut sum: u32 = 0;

    loop {
        let l1 = lines.next();
        let l2 = lines.next();
        let l3 = lines.next();

        if l1.is_none() || l2.is_none() || l3.is_none() {
            return sum;
        }

        let letter = common_letter(l1.unwrap(), l2.unwrap(), l3.unwrap());
        sum += score_letter(letter)
    }
}

fn common_letter(str1: &str, str2: &str, str3: &str) -> char {
    let hash1: HashSet<char> = str1.chars().collect();
    let hash2: HashSet<char> = str2.chars().collect();
    let hash3: HashSet<char> = str3.chars().collect();

    let left: Vec<&char> = hash1.iter()
        .filter(|letter| hash2.contains(letter) && hash3.contains(letter))
        .collect();

    return *left[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve_part2("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw".lines()),
                   70)
    }

    #[test]
    fn test_common_letter() {
        assert_eq!(common_letter("vJrwpWtwJgWrhcsFMMfFFhFp",
                                 "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                                 "PmmdzqPrVvPwwTWBwg"),
                   'r');
        assert_eq!(common_letter("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                                 "ttgJtRGJQctTZtZT",
                                 "CrZsJsPPZsGzwwsLwLmpwMDw"),
                   'Z');
    }
}