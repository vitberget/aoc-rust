use std::time::Instant;
use RockPaperScissor::{Paper, Rock, Scissor};

use crate::rps::{file_to_rps, RockPaperScissor};

pub(crate) fn part1(name: &str, filename: &str) {
    let now = Instant::now();
    let ex = day2_part1(filename);
    let dur = now.elapsed();

    println!("Part1 {} is {} in {:?}", name, ex, dur);
}

fn day2_part1(filename: &str) -> i32 {
    file_to_rps(filename)
        .into_iter()
        .map(score_p1)
        .sum()
}

pub fn score_p1((opponent, player): (RockPaperScissor, RockPaperScissor)) -> i32 {
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).

    let winner_score = match player {
        Rock => match opponent {
            Rock => 3,
            Paper => 0,
            Scissor => 6
        },
        Paper => match opponent {
            Rock => 6,
            Paper => 3,
            Scissor => 0
        },
        Scissor => match opponent {
            Rock => 0,
            Paper => 6,
            Scissor => 3
        }
    };

    // (1 for Rock, 2 for Paper, and 3 for Scissors)
    let move_score = match player {
        Rock => 1,
        Paper => 2,
        Scissor => 3
    };

    winner_score + move_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring() {
        assert_eq!(score_p1((Rock, Paper)), 8);
        assert_eq!(score_p1((Paper, Rock)), 1);
        assert_eq!(score_p1((Scissor, Scissor)), 6);
    }

    #[test]
    #[ignore = "takes filename instead of text, idk to fix"]
    fn test_running() {
        assert_eq!(day2_part1(include_str!("example.txt")), 15);
        assert_eq!(day2_part1(include_str!("example.txt")), 11475);
    }
}
