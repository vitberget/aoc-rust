use std::collections::HashSet;
use std::fs;
use std::str::Lines;

use crate::part1::score_letter;

pub(crate) fn part2(filename: &str) {
    let score = solve_part2(fs::read_to_string(filename).unwrap().lines());

    println!("Part 2 {} = {}", filename, score);
}

fn solve_part2(mut lines: Lines) -> u32 {
    let mut parts: Vec<char> = vec![];

    loop {
        let l1 = lines.next();
        let l2 = lines.next();
        let l3 = lines.next();

        if l1.is_none() || l2.is_none() || l3.is_none() { break; }

        parts.push(common_letter(l1.unwrap(), l2.unwrap(), l3.unwrap()));
    }

    return parts.iter()
        .map(|letter| score_letter(*letter))
        .sum();
}

fn common_letter(str1: &str, str2: &str, str3: &str) -> char {
    let hash1: HashSet<char> = str1.chars().collect();
    let hash2: HashSet<char> = str2.chars().collect();
    let hash3: HashSet<char> = str3.chars().collect();

    let hash12: HashSet<&char> = hash1.intersection(&hash2).collect();

    let left: Vec<&char> = hash3.iter()
        .filter(|letter| hash12.contains(letter))
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