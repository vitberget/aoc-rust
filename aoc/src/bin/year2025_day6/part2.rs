use std::str::Chars;

use aoc_procmacros::aoc_profile;

use crate::part1::Op;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let mut lines: Vec<Chars> = text
        .lines()
        .map(|str| str.chars())
        .collect();

    let line_count = lines.len();

    let mut operator_line: Chars = lines[line_count - 1].clone();

    let mut result:usize = 0;
    let mut current:usize = 0;
    let mut operator: Option<Op> = None;

    loop {
        match operator_line.next() {
            Some('+') => {
                result += current;
                current = 0;
                operator = Some(Op::Addition);
            }
            Some('*') => {
                result += current;
                current = 1;
                operator = Some(Op::Multiplication);
            }
            None => { 
                result += current;
                break;
            }
            _ => {}
        }

        let mut number: Option<usize> = None;
        for line in &mut lines {
            // let ch = line.next();
            if let Some(ch) = line.next() && let Some(digit) = ch.to_digit(10) {
                number = match number {
                    Some(number) => Some(number * 10 + digit as usize),
                    None => Some(digit as usize)
                };
            }
        }

        if let Some(number) = number {
            match operator {
                Some(Op::Addition) => current += number,
                Some(Op::Multiplication) => current *= number,
                _ => {}
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 3263827);
        Ok(())
    }
}
