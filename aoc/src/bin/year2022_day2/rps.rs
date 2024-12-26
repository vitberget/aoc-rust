use std::{fmt, fs};
use RockPaperScissor::{Paper, Rock, Scissor};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RockPaperScissor {
    Rock,
    Paper,
    Scissor,
}

impl fmt::Display for RockPaperScissor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   Rock => "rock",
                   Paper => "paper",
                   Scissor => "scissor"
               })
    }
}

pub(crate) fn file_to_rps(filename: &str) -> Vec<(RockPaperScissor,RockPaperScissor)> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().take(2).collect())
        .map(|strings: Vec<&str>| (str_to_rps(strings[0]), str_to_rps(strings[1])))
        .map(|(opponent, player)| (opponent.unwrap(), player.unwrap()))
        .collect()
}

pub(crate) fn str_to_rps(src: &str) -> Option<RockPaperScissor> {
    match src {
        "A" => Some(Rock),
        "B" => Some(Paper),
        "C" => Some(Scissor),

        "X" => Some(Rock),
        "Y" => Some(Paper),
        "Z" => Some(Scissor),

        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_rps() {
        assert_eq!(str_to_rps("A"), Some(Rock));
        assert_eq!(str_to_rps("B"), Some(Paper));
    }
}
