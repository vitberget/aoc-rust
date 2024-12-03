use std::env::Args;
/// Returns first argument as filename, or a default
pub fn get_aoc_filename(args: Args, year: u16, day: u16) -> String {
    args
        .into_iter()
        .nth(1)
        .unwrap_or(format!("puzzles/year{year}_day{day}.txt"))
}
