use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

use anyhow::bail;
use aoc_utils::get_aoc_filename;
use itertools::Itertools;


pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2018, 2);
    let text = read_to_string(filename)?;
    let text = text.trim();

    let p1 = solve_part_1(text)?;
    println!("Part 1: {p1}");

    let p2 = solve_part_2(text)?;
    println!("Part 2: {p2}");

    Ok(())
}

fn solve_part_1(text: &str) -> anyhow::Result<usize> {
    let lines = text.lines()
        .map(|line| line_count(line).unwrap())
        .collect_vec();

    let twos: usize = lines.iter()
        .filter(|map| map.values().contains(&2))
        .count();

    let threes: usize = lines.iter()
        .filter(|map| map.values().contains(&3))
        .count();

    Ok(twos * threes)
}

fn line_count(line: &str) -> anyhow::Result<HashMap<u8, usize>> {
    let mut result: HashMap<u8, usize> = HashMap::new();

    for b in line.bytes() {
        match result.get_mut(&b) {
            Some(count) => *count +=1,
            None => {
                result.insert(b, 1);
            }
        }
    }

    Ok(result)
}


fn solve_part_2(text: &str) -> anyhow::Result<String> {
    let vector = text.lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let mut lines = vector.as_slice();

    while lines.len() > 1 {
        let check = &lines[0];
        lines = &lines[1..];

        for line in lines {
            let result = check.iter().zip(line.iter())
                .filter(|(a,b)| a == b)
                .collect_vec();

            if result.len() == check.len() - 1 {
                let text: String = result.iter()
                    .map(|(l,_)| *l)
                    .collect();
                return Ok(text);
            }
        }
    }

    bail!("fail")
}
