use std::collections::HashMap;

use aoc_procmacros::aoc_profile;

#[derive(Debug)]
pub enum Op {
    Addition,
    Multiplication
}

#[derive(Debug)]
enum NumOrOp {
    Number(usize),
    Operator(Op)
}

impl From<&str> for NumOrOp {
    fn from(text: &str) -> Self {
        if text == "+" {
            Self::Operator(Op::Addition)
        } else if text == "*" {
            Self::Operator(Op::Multiplication)
        } else {
            Self::Number(text.parse().unwrap())
        }
    }
}

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let mut lists: HashMap<usize, Vec<NumOrOp>> = HashMap::new();
    for line in text.lines() {
        line.split_whitespace()
            .map(NumOrOp::from)
            .enumerate()
            .for_each(|(index, num_or_op)| {
                match lists.get_mut(&index) {
                    Some(list) => { list.push(num_or_op); }
                    None => { lists.insert(index, vec![num_or_op]); }
                };
            }); 
    };

    let result = lists.values()
        .map(|list| process_list(list))
        .sum();
        
    Ok(result)
}

fn process_list(list: &[NumOrOp]) -> usize {
    let mut operator: Option<Op> = None;
    let mut result: usize = 0;
    for op in list.iter().rev() {
        match op {
            NumOrOp::Operator(Op::Addition) => { 
                operator = Some(Op::Addition);
                result = 0;
            }
            NumOrOp::Operator(Op::Multiplication) => { 
                operator = Some(Op::Multiplication);
                result = 1;
            }
            NumOrOp::Number(number) => {
                match operator {
                    Some(Op::Addition) => { result += number; }
                    Some(Op::Multiplication) => { result *= number; }
                    _ => {}
                }
            }
        } 
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = include_str!("example.txt");
        let result = part1(EXAMPLE)?;
        assert_eq!(result, 4277556);
        Ok(())
    }
}
