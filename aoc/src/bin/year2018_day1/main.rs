use std::collections::HashSet;
use std::env::args;
use std::fs::read_to_string;

use aoc_utils::get_aoc_filename;


pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2018, 1);
    let text = read_to_string(filename)?;
    let text = text.trim();

    let p1 = solve_part_1(text)?;
    println!("Part 1: {p1}");

    let p2 = solve_part_2(text)?;
    println!("Part 2: {p2}");

    Ok(())
}

fn solve_part_1(text: &str) -> anyhow::Result<i64> {
    let sum = text.lines()
        .map(|line| line.parse::<i64>().unwrap_or(0))
        .sum();
    Ok(sum)
}

fn solve_part_2(text: &str) -> anyhow::Result<i64> {
    let mut visited: HashSet<i64> = HashSet::new();
    let mut current: i64 = 0;
    visited.insert(0);
    loop {
        for num in text.lines() .map(|line| line.parse::<i64>().unwrap_or(0)) {
            current += num; 
            if !visited.insert(current) { return Ok(current) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1("+1\n+1\n+1").unwrap(), 3);
        assert_eq!(solve_part_1("+1\n+1\n-2").unwrap(), 0);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2("+1\n-1").unwrap(), 0);
        assert_eq!(solve_part_2("+3\n+3\n+4\n-2\n-4").unwrap(), 10);
        assert_eq!(solve_part_2("-6\n+3\n+8\n+5\n-6").unwrap(), 5);
        assert_eq!(solve_part_2("+7\n+7\n-2\n-7\n-4").unwrap(), 14);
    }
}
