use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;

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

    let p2 = solve_part_2(text, 60, 5)?;
    println!("Part 2: {p2}");

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

fn solve_part_2(text: &str, time_offset: usize, num_robots: usize) -> anyhow::Result<usize> {
    let mut deps: HashMap<char, HashSet<char>> = HashMap::new();
    for (l2, l1) in text.lines().map(line_to_dep) {
        deps.entry(l1)
            .and_modify(|set| { set.insert(l2); })
            .or_insert(HashSet::from([l2]));

        deps.entry(l2).or_default();
    }

    let mut answer: String = String::new();
    let mut now: usize = 0;
    let mut robots: Vec<(char, usize)> = vec![];

    while ! deps.is_empty() {
        let complete_robots = robots.iter().filter(|(_, time)| time <= &now).sorted_by(|(l1,_),(l2,_)| l1.cmp(l2));
        for (letter,_) in complete_robots {
            answer.push(*letter);
            deps.remove(letter);
            deps.iter_mut().for_each(|(_,v)| { v.remove(letter); });
        }

        robots.retain(|(_,time)| time > &now);

        while robots.len() < num_robots {
            let robot_letters: HashSet<&char> = robots.iter().map(|(letter,_)| letter).collect();

            let next_letter = deps.iter()
                .filter(|(k,_)| ! robot_letters.contains(k))
                .filter(|(_,v)| v.is_empty())
                .map(|(k,_)| k)
                .sorted()
                .next();

            if let Some(next_letter) = next_letter {
                robots.push((next_letter.to_owned(), now + processing_time(*next_letter, time_offset)));
            } else {
                break;
            }
        }

        now += 1;
    }

    Ok(now - 1)
}

fn processing_time(next_letter: char, time_offset: usize) -> usize {
    next_letter as usize - 'A' as usize +  time_offset + 1
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
    fn test_offset() {
        assert_eq!(processing_time('A', 60), 61);
        assert_eq!(processing_time('Z', 60), 86);
    }

    #[test]
    fn test_part_1() {
        let text = include_str!("example.txt");
        assert_eq!(solve_part_1(text).unwrap(), "CABDFE");
    }

    #[test]
    fn test_part_2() {
        let text = include_str!("example.txt");
        assert_eq!(solve_part_2(text, 0, 2).unwrap(), 15); 
    }
}
