use std::ops::RangeBounds;
use trait_stuff::StartEnd;

mod util;
mod trait_stuff;

fn main() {
    let puzzle = include_str!("../../../puzzles/year2022_day4.txt");

    let part1 = solve_part1(puzzle);
    println!("part1 {part1}");

    let part2 = solve_part2(puzzle);
    println!("part2 {part2}");
}

fn solve_part1(example: &str) -> usize {
    return example.lines()
        .map(|line| util::line_to_ranges(line))
        .filter(|(r1, r2)| overlaps_completely(r1, r2))
        .count();
}

fn solve_part2(example: &str) -> usize {
    return example.lines()
        .map(|line| util::line_to_ranges(line))
        .filter(|(r1, r2)| overlaps_at_all(r1, r2))
        .count();
}

fn overlaps_completely<T: RangeBounds<i32> + StartEnd<i32>>(rng_a: &T, rng_b: &T) -> bool {
    return rng_a.contains(rng_b.the_start()) && rng_a.contains(rng_b.the_end())
        || rng_b.contains(rng_a.the_start()) && rng_b.contains(rng_a.the_end());
}

fn overlaps_at_all<T: RangeBounds<i32> + StartEnd<i32>>(rng_a: &T, rng_b: &T) -> bool {
    return rng_a.contains(rng_b.the_start()) ||
        rng_a.contains(rng_b.the_end()) ||
        rng_b.contains(rng_a.the_start());
    // || p1.contains(&p0.end());p1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_yes() {
        assert_eq!(overlaps_completely(&(2..=8), &(3..=7)), true);
        assert_eq!(overlaps_completely(&(2..=4), &(6..=8)), false);
    }

    #[test]
    fn test1() {
        let example = include_str!("../../../examples/year2022_day4.txt");
        assert_eq!(solve_part1(example), 2)
    }
}

