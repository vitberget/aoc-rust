use std::time::Instant;
use RPS::{PAPER, ROCK, SCISSOR};

use crate::part1::score_p1;
use crate::rps::{file_to_rps, RPS};

pub(crate) fn part2(name: &str, filename: &str) {
    let now = Instant::now();
    let ex = day2_part2(filename);
    let dur = now.elapsed();

    println!("Part2 {} is {} in {:?}", name, ex, dur);
}

fn day2_part2(filename: &str) -> i32 {
    return file_to_rps(filename)
        .into_iter()
        .map(|rps_moves| score_p2(rps_moves))
        .sum();
}

fn score_p2((robot, human): (RPS, RPS)) -> i32 {
    // "Anyway, the second column says how the round needs to end:
    // X means you need to lose, -- rock
    // Y means you need to end the round in a draw, and --paper
    // Z means you need to win. Good luck!" --scissor

    let new_human = match human {
        ROCK => match robot {
            ROCK => SCISSOR,
            PAPER => ROCK,
            SCISSOR => PAPER
        },
        PAPER => robot,
        SCISSOR => match robot {
            ROCK => PAPER,
            PAPER => SCISSOR,
            SCISSOR => ROCK
        }
    };

    return score_p1((robot, new_human));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_scoring() {
        assert_eq!(score_p2((ROCK, PAPER)), 4);
        assert_eq!(score_p2((PAPER, ROCK)), 1);
        assert_eq!(score_p2((SCISSOR, SCISSOR)), 7);
    }
}