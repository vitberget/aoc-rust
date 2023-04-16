use std::env::args;
use std::fs::read_to_string;
use aoc_utils::get_aoc_filename;
use crate::folder_sizes::folder_sizes;

mod folder_sizes;

fn main() {
    let filename = get_aoc_filename(args(), 2022, 7);
    let text = read_to_string(filename).unwrap();

    let dir_sizes = folder_sizes(&text);

    let p1 = part_1(&dir_sizes);
    println!("part 1 {p1}");

    let p2 = part_2(&dir_sizes);
    println!("part 2 {p2}");
}

fn part_1(dir_sizes: &Vec<u32>) -> u32 {
    let limit: u32 = 100_000;
    return dir_sizes
        .iter()
        .filter(|&n| n < &limit)
        .sum();
}

fn part_2(dir_sizes: &Vec<u32>) -> u32 {
    let total_size: &u32 = dir_sizes.iter().max().unwrap();
    let max_size: u32 = 70_000_000 - 30_000_000;

    return *dir_sizes.iter()
        .filter(|&n| total_size - n <= max_size)
        .min()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_ex() {
        let sizes = folder_sizes(include_str!("../../../examples/year2022_day7.txt"));
        let p1 = part_1(&sizes);
        assert_eq!(p1, 95437);
    }

    #[test]
    fn part_2_ex() {
        let sizes = folder_sizes(include_str!("../../../examples/year2022_day7.txt"));
        let p2 = part_2(&sizes);
        assert_eq!(p2, 24933642);
    }
}