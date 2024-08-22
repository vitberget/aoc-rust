use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;


pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2018, 7);
    let text = read_to_string(filename)?;
    let text = text.trim();

    let p1 = solve_part_1(text)?;
    println!("Part 1: {p1}");

    // let p2 = solve_part_2(text)?;
    // println!("Part 2: {p2}");

    Ok(())
}


fn solve_part_1(text: &str) -> anyhow::Result<String> {
    let mut deps: HashMap<char, HashSet<char>> = HashMap::new();
    for (l2, l1) in text.lines().map(line_to_dep) {
        deps.entry(l1)
            .and_modify(|set| { set.insert(l2); })
            .or_insert(HashSet::from([l2]));

        deps.entry(l2).or_default();
    }

    let mut answer: String = String::new();

    while ! deps.is_empty() {
        let next_letter = deps.iter()
            .filter(|(_,v)| v.is_empty())
            .map(|(k,_)| k)
            .sorted()
            .next()
            .unwrap()
            .to_owned();

        deps.remove(&next_letter);
        deps.iter_mut().for_each(|(_,v)| { v.remove(&next_letter); });
        answer.push(next_letter);
    }

    Ok(answer)
}

lazy_static! {
    static ref REGEX: Regex = Regex::new("Step ([A-Z]) must be finished before step ([A-Z]) can begin.*").unwrap();
}
fn line_to_dep(line: &str) -> (char,char) {
    let capture = REGEX.captures(line).unwrap();
    let l1: char = capture[1].chars().next().unwrap();
    let l2: char = capture[2].chars().next().unwrap();

    (l1, l2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dep() {
       assert_eq!(line_to_dep("Step C must be finished before step A can begin."), ('C', 'A'));
    }

    #[test]
    fn test_part_1() {
        let text = include_str!("example.txt");
        assert_eq!(solve_part_1(text).unwrap(), "CABDFE");
    }
}
