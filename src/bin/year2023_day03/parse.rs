use std::collections::HashSet;

use anyhow::Context;

use crate::structs::{Schematic, Position, Number};

pub fn parse_text_to_schematic(text: &str) -> anyhow::Result<Schematic> {
    let mut symbols: HashSet<Position> = HashSet::new();
    let mut numbers: Vec<Number> = vec![];
    let mut current_number: Option<Number> = None;

    for (y, line) in text.lines().enumerate() {
        if let Some(number) = current_number {
            numbers.push(number);
            current_number = None;
        }
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => {
                    if let Some(number) = current_number {
                        numbers.push(number);
                        current_number = None;
                    };
                }
                ch if ch.is_numeric() => {
                    let value = ch.to_digit(10).context("Not a digit")?;
                    let position = Position::new(x,y);
                    match current_number {
                        Some(ref mut number) => {
                            number.number = number.number * 10 + value;
                            number.positions.insert(position);
                        }
                        None => {
                            current_number = Some(Number {
                                number: value,
                                positions: HashSet::from([position])
                            })
                        }
                    };
                }
                _ => { 
                    if let Some(number) = current_number {
                        numbers.push(number);
                        current_number = None;
                    };
                    symbols.insert(Position::new(x, y)); }
            }
        }
    }

    if let Some(number) = current_number {
        numbers.push(number);
    }

    Ok(Schematic { numbers, symbols })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
       let text = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
       let _schematic = parse_text_to_schematic(&text);
       // println!("Schematic: {:?}", schematic);
    }
}
