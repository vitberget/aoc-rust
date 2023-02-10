use std::fs;
use std::time::{Instant};
use crate::part1::score_p1;
use crate::rps::{RPS, str_to_rps};

pub(crate) fn part2(name: &str, filename: &str) {
    let now = Instant::now();
    let ex = day2_part2(filename);
    let dur = now.elapsed();

    println!("Part2 {} is {} in {:?}", name, ex, dur);
}

fn day2_part2(filename: &str) -> i32 {
    return fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().take(2).collect())
        .map(|strings: Vec<&str>| (str_to_rps(strings[0]), str_to_rps(strings[1])))
        .map(|(m1, m2)| (m1.unwrap(), m2.unwrap()))
        .map(|rps_moves| score_p2(rps_moves))
        .sum();
}

fn score_p2((robot, human): (RPS, RPS)) -> i32 {
    // "Anyway, the second column says how the round needs to end:
    // X means you need to lose, -- rock
    // Y means you need to end the round in a draw, and --paper
    // Z means you need to win. Good luck!" --scissor

    let new_human = match human {
        RPS::ROCK => match robot {
            RPS::ROCK => RPS::SCISSOR,
            RPS::PAPER => RPS::ROCK,
            RPS::SCISSOR => RPS::PAPER
        },
        RPS::PAPER => robot,
        RPS::SCISSOR => match robot {
            RPS::ROCK => RPS::PAPER,
            RPS::PAPER => RPS::SCISSOR,
            RPS::SCISSOR => RPS::ROCK
        }
    };

    return score_p1((robot,new_human))
}