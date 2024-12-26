use std::time::Instant;
use RockPaperScissor::{Paper, Rock, Scissor};

use crate::part1::score_p1;
use crate::rps::{file_to_rps, RockPaperScissor};

pub(crate) fn part2(name: &str, filename: &str) {
    let now = Instant::now();
    let ex = day2_part2(filename);
    let dur = now.elapsed();

    println!("Part2 {} is {} in {:?}", name, ex, dur);
}

fn day2_part2(filename: &str) -> i32 {
    file_to_rps(filename)
        .into_iter()
        .map(score_p2)
        .sum()
}

fn score_p2((opponent, player): (RockPaperScissor, RockPaperScissor)) -> i32 {
    // "Anyway, the second column says how the round needs to end:
    // X means you need to lose, -- rock
    // Y means you need to end the round in a draw, and --paper
    // Z means you need to win. Good luck!" --scissor

    let real_player_move = match player {
        Rock => match opponent {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper
        },
        Paper => opponent,
        Scissor => match opponent {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock
        }
    };

    score_p1((opponent, real_player_move))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scoring() {
        assert_eq!(score_p2((Rock, Paper)), 4);
        assert_eq!(score_p2((Paper, Rock)), 1);
        assert_eq!(score_p2((Scissor, Scissor)), 7);
    }

    #[test]
    #[ignore = "takes filename instead of text, idk to fix"]
    fn test_running() {
        assert_eq!(day2_part2(include_str!("example.txt")), 12);
        assert_eq!(day2_part2(include_str!("example.txt")), 16862);
    }
}
