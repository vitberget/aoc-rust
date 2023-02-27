use std::{fmt, fs};
use RPS::{PAPER, ROCK, SCISSOR};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPS {
    ROCK,
    PAPER,
    SCISSOR,
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   ROCK => "rock",
                   PAPER => "paper",
                   SCISSOR => "scissor"
               })
    }
}

pub(crate) fn file_to_rps(filename: &str) -> Vec<(RPS,RPS)> {
    return fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().take(2).collect())
        .map(|strings: Vec<&str>| (str_to_rps(strings[0]), str_to_rps(strings[1])))
        .map(|(opponent, player)| (opponent.unwrap(), player.unwrap()))
        .collect()
}

pub(crate) fn str_to_rps(src: &str) -> Option<RPS> {
    return match src {
        "A" => Some(ROCK),
        "B" => Some(PAPER),
        "C" => Some(SCISSOR),

        "X" => Some(ROCK),
        "Y" => Some(PAPER),
        "Z" => Some(SCISSOR),

        _ => None
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_rps() {
        assert_eq!(str_to_rps("A"), Some(ROCK));
        assert_eq!(str_to_rps("B"), Some(PAPER));
    }
}