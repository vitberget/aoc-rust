use std::fs::read_to_string;
use std::env::args;

use aoc_utils::get_aoc_filename;
use itertools::Itertools;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2024, 2);
    let text = read_to_string(filename)?;
    let numbers = get_numbers(text); 

    let part1 = part1(&numbers);
    println!("Part1: {part1}");

    let part2 = part2(&numbers);
    println!("Part2: {part2}");

    Ok(())
}

fn part1(numbers: &[Vec<i64>]) -> usize {
    numbers.iter()
        .filter(|numbers: &&std::vec::Vec<i64>| is_safe(numbers))
        .count()
}

fn part2(numbers: &[Vec<i64>]) -> usize {
    numbers.iter()
        .filter(|numbers: &&std::vec::Vec<i64>| is_tolerable_safe(numbers))
        .count()
}

fn is_tolerable_safe(numbers: &[i64]) -> bool {
    if is_safe(numbers) { return true }

    for split in 0..(numbers.len()) {
        let head = &numbers[..split];
        let tail = &numbers[(split+1)..];
        let numbers = [head, tail].concat();

        if is_safe(&numbers) { return true }
    }
    false
}

fn is_safe(numbers: &[i64]) -> bool {
    let diffs = numbers.windows(2)
        .map(|pair| pair[0] - pair[1])
        .collect_vec();

    for diff in &diffs {
        if *diff == 0 { return false }
        if *diff < -3 { return false; }
        if *diff > 3 { return false }
    }

    for (a,b) in diffs.iter().map(|diff| diff.is_positive()).tuple_windows() {
        if a != b { return false }
    }

    true
}

fn get_numbers(text: String) -> Vec<Vec<i64>> {
    text.lines()
        .map(line_to_numbers)
        .collect()

}

fn line_to_numbers(line: &str) -> Vec<i64> {
    line.split_whitespace().map(|word| word.parse().unwrap()).collect()
}
