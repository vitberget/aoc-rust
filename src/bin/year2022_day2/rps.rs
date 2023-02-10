use std::{fmt, fs};

#[derive(Clone, Copy)]
pub enum RPS {
    ROCK,
    PAPER,
    SCISSOR,
}

impl fmt::Display for RPS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
               match self {
                   RPS::ROCK => "rock",
                   RPS::PAPER => "paper",
                   RPS::SCISSOR => "scissor"
               })
    }
}

pub(crate) fn str_to_rps(src: &str) -> Option<RPS> {
    return match src {
        "A" => Some(RPS::ROCK),
        "B" => Some(RPS::PAPER),
        "C" => Some(RPS::SCISSOR),

        "X" => Some(RPS::ROCK),
        "Y" => Some(RPS::PAPER),
        "Z" => Some(RPS::SCISSOR),

        _ => None
    };
}
pub(crate) fn file_to_rps(filename: &str) -> Vec<(RPS,RPS)> {
    return fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|line| line.split_whitespace().take(2).collect())
        .map(|strings: Vec<&str>| (str_to_rps(strings[0]), str_to_rps(strings[1])))
        .map(|(m1, m2)| (m1.unwrap(), m2.unwrap()))
        .collect()
}
