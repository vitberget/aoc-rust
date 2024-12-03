use std::collections::{VecDeque};
use regex::Regex;

pub(crate) type Stack = VecDeque<char>;
pub(crate) type Stacks = Vec<Stack>;
type Move = (u32, u32, u32);
type Moves = Vec<Move>;

pub(crate) fn parse_text(text: &str) -> (Stacks, Moves) {
    let mut splitted = text.split("\n\n");

    let stacks = parse_stack(splitted.next().unwrap());
    let moves = parse_moves(splitted.next().unwrap());

    (stacks, moves)
}

fn parse_moves(move_split: &str) -> Moves {
    let move_regex: Regex = Regex::new(r"^move (?P<amount>[0-9]+) from (?P<from>[0-9]+) to (?P<to>[0-9])$").unwrap();
    return move_split
        .lines()
        .map(|line| parse_move(&move_regex, line))
        .collect();
}

fn parse_stack(stack_text: &str) -> Stacks {
    let num_stacks = get_no_of_stacks(stack_text);

    let mut stacks: Stacks = vec![VecDeque::new(); num_stacks];

    for line in stack_text.lines() {
        let mut idx: usize = 0;
        let mut line = line;

        while !line.is_empty() && !line.starts_with(" 1 ") {
            let (part, rest) = if line.len() > 3 { line.split_at(4) } else { (line, "") };

            let c = part.chars().nth(1).unwrap();
            if !c.is_ascii_whitespace() {
                stacks[idx].push_back(c);
            }

            idx += 1;
            line = rest;
        }
    }

    stacks
}

fn get_no_of_stacks(stack_text: &str) -> usize {
    return stack_text.lines().next_back()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
}

fn parse_move(move_regex: &Regex, line: &str) -> (u32, u32, u32) {
    let pokemon = move_regex.captures(line).unwrap();
    let amount: u32 = pokemon["amount"].parse().unwrap();
    let from: u32 = pokemon["from"].parse().unwrap();
    let to: u32 = pokemon["to"].parse().unwrap();
    (amount, from, to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_example() {
        let (stacks, moves) = parse_text(include_str!("../../../examples/year2022_day5.txt"));
        assert_eq!(stacks, vec![
            Stack::from(['N', 'Z']),
            Stack::from(['D', 'C', 'M']),
            Stack::from(['P']),
        ]);
        assert_eq!(moves, vec![
            (1, 2, 1),
            (3, 1, 3),
            (2, 2, 1),
            (1, 1, 2),
        ]);
    }
}