use std::io;
use part1::part1;
use part2::part2;

mod rps;
mod part1;
mod part2;

fn main() {
    println!("Year 2022, day 2");
    part1("Example", "examples/year2022-day2.txt");
    part1("Puzzle", "puzzles/year2022-day2.txt");

    part2("Example", "examples/year2022-day2.txt");
    part2("Puzzle", "puzzles/year2022-day2.txt");
}

