use std::collections::HashSet;

use crate::structs::{Schematic, Number, Position};

pub fn solve_part_1(schematic: &Schematic) -> anyhow::Result<u32> {
    let symbols_space: HashSet<Position> = schematic.symbols.iter()
        .flat_map(|pos| pos.surrounding())
        .collect();

    Ok(schematic.numbers.iter()
        .filter(|number| is_number_adjecent_to_symbol(number, &symbols_space))
        .map(|number| number.number)
        .sum())
}

pub fn is_number_adjecent_to_symbol(number: &&Number, symbols: &HashSet<Position>) -> bool {
    for position in &number.positions {
        if symbols.contains(position) {
            return true;
        }
    }
    false
}
