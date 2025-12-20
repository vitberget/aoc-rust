use aoc_procmacros::aoc_profile;
use itertools::Itertools;

use crate::part1::{parse::parse_input, structs::{requirements::Requirements, shape::Shape}};

pub mod structs;
pub mod parse;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let (shapes, requirements) = parse_input(text)?;

    let shapes_variations = shapes.iter()
        .map(|shape| shape.variations())
        .collect_vec();

    let result = requirements.iter()
        .filter(|req| is_requirement_fulfilled(req, &shapes_variations))
        .count();

    Ok(result)
}

fn is_requirement_fulfilled(req: &Requirements, shapes_variations: &Vec<Vec<Shape>>) -> bool {
    let border = req.get_border();

    todo!()
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let result = part1(include_str!("example.txt"))?;
        assert_eq!(result, 2);
        Ok(())
    }
}



