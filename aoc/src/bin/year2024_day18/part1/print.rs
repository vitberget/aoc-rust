use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

use aoc_utils::color;

use super::{MemPos, Memory};

pub fn print_it(memory: &Memory, visited: &HashSet<MemPos>, current_positions: &HashSet<MemPos>, delay: u64) {
    println!();
    let mut text: String = String::new();
    text.push_str(color::RED);
    text.push_str(&color::goto(0,0));
    text.push('\n');

    for y in 0..=memory.height {
        for x in 0..=memory.width {
            let mp = MemPos { x, y };
            if mp == memory.start_pos {
                text.push_str(color::YELLOW);
                text.push('S');
            } else if mp == memory.end_pos {
                text.push_str(color::YELLOW);
                text.push('E');
            } else if current_positions.contains(&mp) {
                text.push_str(color::WHITE);
                text.push('O');
            } else if visited.contains(&mp) {
                text.push_str(color::LIGHT_GRAY);
                text.push('O');
            } else if memory.corrupted.contains(&mp) {
                text.push_str(color::LIGHT_RED);
                text.push('#');
            } else {
                text.push(' ');
            }
        }
        text.push('\n');
    }
    text.push_str(color::RESET);
    println!("{text}");
    sleep(Duration::from_millis(delay));
}

