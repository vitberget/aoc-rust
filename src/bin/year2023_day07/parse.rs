use anyhow::bail;

use crate::structs::{Plays, Play, Card};

pub fn text_to_plays(text: &str) -> anyhow::Result<Plays> {
    let mut plays: Plays = vec![];

    for line in text.lines() {
        plays.push(line_to_play(line)?);
    }

    Ok(plays)
}

fn line_to_play(line: &str) -> anyhow::Result<Play> {
    let mut iter = line.split_whitespace();
    
    let mut cards = iter.next().unwrap().chars();
    let bet = iter.next().unwrap();

    Ok(Play {
        cards: [
            cards.next().unwrap().try_into()?,
            cards.next().unwrap().try_into()?,
            cards.next().unwrap().try_into()?,
            cards.next().unwrap().try_into()?,
            cards.next().unwrap().try_into()?,
        ],
        bet: bet.parse().unwrap()
    })
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, anyhow::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::C10),
            '2' => Ok(Self::C2),
            '3' => Ok(Self::C3),
            '4' => Ok(Self::C4),
            '5' => Ok(Self::C5),
            '6' => Ok(Self::C6),
            '7' => Ok(Self::C7),
            '8' => Ok(Self::C8),
            '9' => Ok(Self::C9),

            c => bail!("Not a card {}", c)
        }
    }
}
