use crate::part_1::is_number_adjecent_to_symbol;
use crate::structs::{Schematic, Number, Position};

pub fn solve_part_2(schematic: &Schematic) -> anyhow::Result<u32> {
    Ok(schematic.symbols.iter()
        .map(|symbol_pos| get_numbers_adjacent_to_symbol(symbol_pos, &schematic.numbers))
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| multiply_numbers(numbers))
        .sum())
}

fn get_numbers_adjacent_to_symbol<'a>(symbol_pos: &'a Position, numbers: &'a [Number]) -> Vec<&'a Number> {
    let symbol_space = symbol_pos.surrounding();
    numbers.iter()
        .filter(|number| is_number_adjecent_to_symbol(number, &symbol_space))
        .collect()
}

fn multiply_numbers(numbers: Vec<&Number>) -> u32 {
    numbers.iter()
        .map(|number| number.number)
        .reduce(|a,n| a*n)
        .unwrap_or(0)
}
