use lazy_static::lazy_static;
use regex::Regex;
use std::collections::BTreeMap;

use crate::part_1::hash;

#[derive(Debug, PartialEq)]
struct Lens {
    pub name: String,
    pub focal_lenght: u8,
}

impl Lens {
    fn new(name: impl Into<String>, focal_lenght: u8) -> Self {
        Self { name: name.into(), focal_lenght }
    }
}

pub fn solve_part_2(text: &str) -> anyhow::Result<u32> {
    Ok(do_the_hashmap(text)?.iter()
       .filter(|(_, lenses)| !lenses.is_empty())
       .map(|(bx, lenses)| score_box(bx, lenses))
       .sum())
}

fn score_box(bx: &u8, lenses: &[Lens]) -> u32 {
    let bx: u32 = (*bx as u32) + 1;
    lenses.iter().enumerate()
        .map(|(lens_idx, lens)| {
            let lens_idx: u32 = (lens_idx + 1) as u32;
            let lens_fl: u32 = lens.focal_lenght as u32;
            bx * lens_idx * lens_fl
        }).sum()
}

fn do_the_hashmap(text: &str) -> anyhow::Result<BTreeMap<u8, Vec<Lens>>> {
    let mut result: BTreeMap<u8, Vec<Lens>> = BTreeMap::new();
    (0..=u8::MAX).for_each(|bx| { result.insert(bx, vec![]); });

    text.split(',')
        .map(part_to_instructions)
        .for_each(|(bx, operator, lens)| {
            let lenses = result.get_mut(&bx).unwrap();

            if operator == '-' {
                lenses.retain(|l| l.name != lens.name);
            } else {
                match lenses.iter_mut().find(|l| l.name == lens.name) {
                    Some(l) => { l.focal_lenght = lens.focal_lenght; }
                    None => {lenses.push(lens);}
                }
            };
        });

    Ok(result)
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(r"(\w+)([-=])(\d*)").unwrap();
}

fn part_to_instructions(part: &str) -> (u8, char, Lens) {
    let captures = INSTRUCTION_REGEX.captures(part).unwrap();

    let name: String = captures[1].to_string();
    let operator: char = captures[2].parse().unwrap();

    let value: u8 = captures.get(3)
        .map(|val| val.as_str().parse().unwrap_or(0))
        .unwrap_or(0);

    let bx = hash(&name);

    (bx, operator, Lens::new(name, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_to_instruction_replace() {
        assert_eq!(part_to_instructions("rn=1"), (0, '=', Lens::new("rn",1)));
    }

    #[test]
    fn test_part_to_instruction_delete() {
        assert_eq!(part_to_instructions("cm-"), (3, '-', Lens::new("cm", 0)));
    }

    #[test]
    fn test_year2023_day15_part_2_example() {
        assert_eq!(solve_part_2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").unwrap(), 145);
    }
}
