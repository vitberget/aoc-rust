use std::time::Instant;
use RPS::{PAPER, ROCK, SCISSOR};

use crate::rps::{file_to_rps, RPS};

pub(crate) fn part1(name: &str, filename: &str) {
    let now = Instant::now();
    let ex = day2_part1(filename);
    let dur = now.elapsed();

    println!("Part1 {} is {} in {:?}", name, ex, dur);
}

fn day2_part1(filename: &str) -> i32 {
    return file_to_rps(filename)
        .into_iter()
        .map(|rps_moves| score_p1(rps_moves))
        .sum();
}

pub fn score_p1((robot, human): (RPS, RPS)) -> i32 {
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).

    let winner_score = match human {
        ROCK => match robot {
            ROCK => 3,
            PAPER => 0,
            SCISSOR => 6
        },
        PAPER => match robot {
            ROCK => 6,
            PAPER => 3,
            SCISSOR => 0
        },
        SCISSOR => match robot {
            ROCK => 0,
            PAPER => 6,
            SCISSOR => 3
        }
    };

    // (1 for Rock, 2 for Paper, and 3 for Scissors)
    let move_score = match human {
        ROCK => 1,
        PAPER => 2,
        SCISSOR => 3
    };

    return winner_score + move_score;
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use super::*;

    #[test]
    fn test_scoring() {
        assert_eq!(score_p1((ROCK, PAPER)), 8);
        assert_eq!(score_p1((PAPER, ROCK)), 1);
        assert_eq!(score_p1((SCISSOR, SCISSOR)), 6);
    }

    #[test]
    fn test_running() {
        let d = PathBuf::from(env!("CARGO_MANIFEST_DIR")).display().to_string();
        assert_eq!(day2_part1((d.clone() + "/examples/year2022-day2.txt").as_str()), 15);
        assert_eq!(day2_part1((d.clone() + "/puzzles/year2022-day2.txt").as_str()), 11475);
    }
}