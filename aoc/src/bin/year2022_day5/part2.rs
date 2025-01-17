use std::collections::VecDeque;
use crate::parse_text::parse_text;
use crate::util::stacks_to_answer;

pub(crate) fn part2(text: &str) -> String {
    let (mut stacks, moves) = parse_text(text);

    for (amount, from, to) in moves {
        let from = (from - 1) as usize;
        let to = (to - 1) as usize;

        let i_like_to_move_it: VecDeque<char> = (0..amount)
            .map(|_| stacks[from].pop_front().unwrap())
            .collect();

        for c in i_like_to_move_it.iter().rev() {
            stacks[to].push_front(*c);
        }
    }

    stacks_to_answer(&mut stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let res = part2(include_str!("example.txt"));
        assert_eq!(res, "MCD");
    }
}