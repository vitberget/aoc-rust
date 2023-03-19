use crate::parse_text::{parse_text};

pub(crate) fn part1(text: &str) -> String {
    let (mut stacks, moves) = parse_text(text);

    for (amount, from, to) in moves {
        for _ in 0..amount {
            let c = stacks[(from - 1) as usize].pop_front().unwrap();
            stacks[(to - 1) as usize].push_front(c);
        }
    }

    return stacks.iter()
        .map(|stack| stack.get(0).unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let res = part1(include_str!("../../../examples/year2022_day5.txt"));
        assert_eq!(res, "CMZ");
    }
}