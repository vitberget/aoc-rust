use std::ops::RangeBounds;
use trait_stuff::StartEnd;

mod util;
mod trait_stuff;

fn main() {
    let puzzle = include_str!("../../../../puzzles/year2022_day4.txt");

    let part1 = solve_part1(puzzle);
    println!("part1 {part1}");

    let part2 = solve_part2(puzzle);
    println!("part2 {part2}");
}

fn solve_part1(example: &str) -> usize {
    example.lines()
        .map(util::line_to_ranges)
        .filter(|(r1, r2)| overlaps_completely(r1, r2))
        .count()
}

fn solve_part2(example: &str) -> usize {
    example.lines()
        .map(util::line_to_ranges)
        .filter(|(r1, r2)| overlaps_at_all(r1, r2))
        .count()
}

fn overlaps_completely<T: RangeBounds<i32> + StartEnd<i32>>(rng_a: &T, rng_b: &T) -> bool {
    rng_a.contains(rng_b.the_start()) && rng_a.contains(rng_b.the_end())
        || rng_b.contains(rng_a.the_start()) && rng_b.contains(rng_a.the_end())
}

fn overlaps_at_all<T: RangeBounds<i32> + StartEnd<i32>>(rng_a: &T, rng_b: &T) -> bool {
    rng_a.contains(rng_b.the_start()) ||
        rng_a.contains(rng_b.the_end()) ||
        rng_b.contains(rng_a.the_start())
    // || p1.contains(&p0.end());p1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_yes() {
        assert!(overlaps_completely(&(2..=8), &(3..=7)));
        assert!(!overlaps_completely(&(2..=4), &(6..=8)));
    }

    #[test]
    fn test1() {
        let example = include_str!("example.txt");
        assert_eq!(solve_part1(example), 2)
    }
}

