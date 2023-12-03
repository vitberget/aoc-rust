use std::env::args;
use std::fs::read_to_string;

use aoc_utils::get_aoc_filename;

pub fn main() -> anyhow::Result<()> {
    let filename = get_aoc_filename(args(), 2023, 1);
    let text = read_to_string(filename)?;

    let p1 = solve(&text, line_part_1)?;
    println!("Part 1: {p1}");

    let p2 = solve(&text, line_part_2)?;
    println!("Part 2: {p2}");

    Ok(())
}

fn solve(text: &str, part_fn: fn(&str) -> Vec<u8>) -> anyhow::Result<u32>  {
    Ok(text.lines()
       .map(part_fn)
       .map(do_the_numbers)
       .sum())
}

fn do_the_numbers(numbers: Vec<u8>) -> u32 {
    let digit_1:u32 = *numbers.first().unwrap() as u32;
    let digit_2:u32 = *numbers.last().unwrap() as u32;

    digit_1 * 10 + digit_2
}

fn line_part_1(line: &str) -> Vec<u8> {
    line.chars()
        .filter(|char| char.is_numeric())
        .map(|char| char.to_digit(10))
        .map(|num| num.unwrap())
        .map(|num| num as u8)
        .collect()
}

fn line_part_2(line: &str) -> Vec<u8> {
    let mut line = line;
    let mut result: Vec<u8>= vec![];

    while !line.is_empty() {
        match line {
            line if line.starts_with('0') => result.push(0),
            line if line.starts_with('1') => result.push(1),
            line if line.starts_with('2') => result.push(2),
            line if line.starts_with('3') => result.push(3),
            line if line.starts_with('4') => result.push(4),
            line if line.starts_with('5') => result.push(5),
            line if line.starts_with('6') => result.push(6),
            line if line.starts_with('7') => result.push(7),
            line if line.starts_with('8') => result.push(8),
            line if line.starts_with('9') => result.push(9),
           
            line if line.starts_with("zero") => result.push(0),
            line if line.starts_with("one") => result.push(1),
            line if line.starts_with("two") => result.push(2),
            line if line.starts_with("three") => result.push(3),
            line if line.starts_with("four") => result.push(4),
            line if line.starts_with("five") => result.push(5),
            line if line.starts_with("six") => result.push(6),
            line if line.starts_with("seven") => result.push(7),
            line if line.starts_with("eight") => result.push(8),
            line if line.starts_with("nine") => result.push(9),
            _ => {}
        }

        line = &line[1..];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
       let example = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
       assert_eq!(solve(&example.to_string(), line_part_1).unwrap(), 142);
    }

    #[test]
    fn test_part_1_lines() {
        assert_eq!(line_part_1("1abc2"), vec![1,2]);
        assert_eq!(line_part_1("pqr3stu8vwx"), vec![3,8]);
        assert_eq!(line_part_1("a1b2c3d4e5f"), vec![1,2,3,4,5]);
        assert_eq!(line_part_1("treb7uchet"), vec![7]);
    }

    #[test]
    fn test_do_numbers() {
        assert_eq!(do_the_numbers(vec![1,2]), 12);
        assert_eq!(do_the_numbers(vec![3,8]), 38);
        assert_eq!(do_the_numbers(vec![1,2,3,4,5]), 15);
        assert_eq!(do_the_numbers(vec![7]), 77);
    }

    #[test]
    fn test_part_2_lines() {
        assert_eq!(do_the_numbers(line_part_2("two1nine")), 29);
        assert_eq!(do_the_numbers(line_part_2("eightwothree")), 83);
        assert_eq!(do_the_numbers(line_part_2("abcone2threexyz")), 13);
        assert_eq!(do_the_numbers(line_part_2("xtwone3four")), 24);
        assert_eq!(do_the_numbers(line_part_2("4nineeightseven2")), 42);
        assert_eq!(do_the_numbers(line_part_2("zoneight234")), 14);
        assert_eq!(do_the_numbers(line_part_2("7pqrstsixteen")), 76);
    }
}
