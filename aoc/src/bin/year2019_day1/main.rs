use std::{env::args, fs::read_to_string};

use aoc_utils::get_aoc_filename;



pub fn main() -> anyhow::Result<()> {

    let filename = get_aoc_filename(args(), 2019, 1);
    let text = read_to_string(filename)?;

    let p1 = solve_part_1(&text)?;
    println!("part 1: {p1}");

    let p2 = solve_part_2(&text)?;
    println!("part 2: {p2}");


    Ok(())
}

fn solve_part_1(text: &str) -> anyhow::Result<u32> {
    let result = text.lines()
        .map(|line| line.parse::<u32>().unwrap_or(0))
        .map(get_required_fuel_part1)
        .sum();
    Ok(result)
}

fn solve_part_2(text: &str) -> anyhow::Result<u32> {
    let result = text.lines()
        .map(|line| line.parse::<u32>().unwrap_or(0))
        .map(get_required_fuel_part2)
        .sum();
    Ok(result)
}

fn get_required_fuel_part1(mass: u32) -> u32 {
    (mass/3) - 2
}

fn get_required_fuel_part2(mass: u32) -> u32 {
    let mut total_fuel: u32 = 0;
    let mut step = mass;

    loop {
        step /= 3; 
        if step <= 2 { return total_fuel }
        step -= 2;
        total_fuel += step;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_fuel_part1() {
       assert_eq!(get_required_fuel_part1(12), 2);
       assert_eq!(get_required_fuel_part1(14), 2);
       assert_eq!(get_required_fuel_part1(1969), 654);
       assert_eq!(get_required_fuel_part1(100756), 33583);
    }

    #[test]
    fn test_required_fuel_part2() {
       assert_eq!(get_required_fuel_part2(12), 2);
       assert_eq!(get_required_fuel_part2(1969), 966);
       assert_eq!(get_required_fuel_part2(100756), 50346);
    }
}
