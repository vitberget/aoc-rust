use std::collections::HashSet;
use std::fs;
use std::str::Lines;
use std::time::Instant;

pub(crate) fn part1(filename: &str) {
    let now = Instant::now();
    let score = solve_part1(fs::read_to_string(filename).unwrap().lines());
    let dur = now.elapsed();

    println!("Part 1 {} = {} in {:?}", filename, score, dur);
}

fn solve_part1(lines: Lines) -> u32 {
    return lines.map(|line| common_letter(line))
        .map(|c| score_letter(c))
        .sum();
}

fn common_letter(line: &str) -> char {
    let (left, right) = line.split_at(line.len() / 2);

    let left_hash: HashSet<char> = left.chars().collect();
    let right_hash: HashSet<char> = right.chars().collect();

    return *left_hash.intersection(&right_hash).next().unwrap();
}

pub(crate) fn score_letter(c: char) -> u32 {
    return if c.is_uppercase() {
        (c as u32) - ('A' as u32) + 27
    } else {
        (c as u32) - ('a' as u32) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_example() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(solve_part1(example.lines()), 157);
    }


    #[test]
    fn test_common_letter() {
        assert_eq!(common_letter("vJrwpWtwJgWrhcsFMMfFFhFp"), 'p');
        assert_eq!(common_letter("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), 'L');
        assert_eq!(common_letter("PmmdzqPrVvPwwTWBwg"), 'P');
        assert_eq!(common_letter("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"), 'v');
        assert_eq!(common_letter("ttgJtRGJQctTZtZT"), 't');
        assert_eq!(common_letter("CrZsJsPPZsGzwwsLwLmpwMDw"), 's');
    }

    #[test]
    fn test_score_letter() {
        assert_eq!(score_letter('p'), 16);
        assert_eq!(score_letter('L'), 38);
        assert_eq!(score_letter('P'), 42);
        assert_eq!(score_letter('v'), 22);
        assert_eq!(score_letter('t'), 20);
        assert_eq!(score_letter('s'), 19);
    }
}