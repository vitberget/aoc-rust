use std::collections::{HashMap, HashSet};

use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let network = parse_network(text);
    let mut memory: Vec<&str> = vec!["you"];
    let mut total: usize = 0;

    while !memory.is_empty() {
        let mut new_memory: Vec<&str>= vec![];
        for key in memory {
            if let Some(values) = network.get(key) {
                for val in values {
                    if *val == "out" {
                        total += 1;
                    } else {
                        new_memory.push(*val);
                    }
                }
            }
        }
        memory = new_memory;
    }
    Ok(total)
}

fn parse_network(text: &str) -> HashMap<&str, HashSet<&str>> {
    text.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (&str, HashSet<&str>) {
    let mut big_split = line.split(":");
    let key = big_split.next().unwrap().trim();
    let values = big_split.next().unwrap()
        .split_whitespace()
        .map(|word| word.trim())
        .collect();
    (key, values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let result = part1(include_str!("example.txt"))?;
        assert_eq!(result, 5);
        Ok(())
    }
}
