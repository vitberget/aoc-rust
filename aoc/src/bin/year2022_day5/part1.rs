use crate::parse_text::parse_text;
use crate::util::stacks_to_answer;

pub(crate) fn part1(text: &str) -> String {
    let (mut stacks, moves) = parse_text(text);

    for (amount, from, to) in moves {
        let from = (from - 1) as usize;
        let to = (to - 1) as usize;

        for _ in 0..amount {
            let c = stacks[from].pop_front().unwrap();
            stacks[to].push_front(c);
        }
    }

    stacks_to_answer(&mut stacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let res = part1(include_str!("../../../../examples/year2022_day5.txt"));
        assert_eq!(res, "CMZ");
    }
}