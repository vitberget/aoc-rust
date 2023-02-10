use std::fmt;

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