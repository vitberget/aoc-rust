use std::fs;
use std::time::{Instant};
use crate::rps::{RPS, str_to_rps};


pub(crate) fn part1(name: &str, filename: &str) {
    let now = Instant::now();
    let ex = day2_part1(filename);
    let dur = now.elapsed();

    println!("Part1 {} is {} in {:?}", name, ex, dur);
}

fn day2_part1(filename: &str) -> i32 {
    return fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().take(2).collect())
        .map(|strings: Vec<&str>| (str_to_rps(strings[0]), str_to_rps(strings[1])))
        .map(|(m1,m2)| (m1.unwrap(), m2.unwrap()))
        .map(|rps_moves| score_p1(rps_moves))
        .sum();
}

pub fn score_p1((robot, human): (RPS, RPS)) -> i32 {
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).

    let winner_score = match human {
        RPS::ROCK => match robot {
            RPS::ROCK => 3,
            RPS::PAPER => 0,
            RPS::SCISSOR => 6
        },
        RPS::PAPER => match robot {
            RPS::ROCK => 6,
            RPS::PAPER => 3,
            RPS::SCISSOR => 0
        },
        RPS::SCISSOR => match robot {
            RPS::ROCK => 0,
            RPS::PAPER => 6,
            RPS::SCISSOR => 3
        }
    };

    // (1 for Rock, 2 for Paper, and 3 for Scissors)
    let move_score = match human {
        RPS::ROCK => 1,
        RPS::PAPER => 2,
        RPS::SCISSOR => 3
    };

    return winner_score + move_score;
}