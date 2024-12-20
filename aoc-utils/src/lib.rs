use std::env::Args;

pub mod color;
pub mod char_map;

/// Returns first argument as filename, or a default
pub fn get_aoc_filename(args: Args, year: u16, day: u16) -> String {
    args
        .into_iter()
        .nth(1)
        .unwrap_or(format!("puzzles/year{year}_day{day}.txt"))
}

#[macro_export]
/// Use first argument as file if provided, otherwise use filename based on module_path
macro_rules! aoc_puzzle {
    () => {{
        let __file_name = std::env::args().into_iter()
            .nth(1)
            .unwrap_or(format!("puzzles/{}.txt", module_path!()));
        std::fs::read_to_string(__file_name)
    }};
}

