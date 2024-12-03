use regex::Regex;
use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 3);
    let text = read_to_string(filename)?;

    let part1 = part_1(&text);
    println!("Part 1: {part1}");

    let part2 = part_2(&text);
    println!("Part 2: {part2}");

    Ok(())
}

fn part_1(text: &str) -> u32 {
    Regex::new(r"(mul\((\d+),(\d+)\))").unwrap()
        .captures_iter(text)
        .map(|capture|{
            let (_,[_,n1, n2]) = capture.extract();
            n1.parse::<u32>().unwrap() * n2.parse::<u32>().unwrap()
        }).sum()
}

fn part_2(text: &str) -> u32 {
    Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|don't\(\)").unwrap()
        .captures_iter(text)
        .fold((true, 0_u32), |acc, capture| 
            match capture.get(0).unwrap().as_str() {
                "do()" => (true, acc.1),
                "don't()" => (false, acc.1),
                _ => parse_mul(acc, capture)
            }).1
}

fn parse_mul(acc: (bool, u32), capture: regex::Captures) -> (bool, u32) {
    if acc.0 {
        let n1:u32 = capture.get(2).unwrap().as_str().parse().unwrap();
        let n2:u32 = capture.get(3).unwrap().as_str().parse().unwrap();
        (acc.0, acc.1 + n1 * n2)
    } else {
        acc
    }
}
