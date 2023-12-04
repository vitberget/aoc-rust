use std::collections::HashSet;

use crate::structs::Card;

pub fn parse(text: &str) -> anyhow::Result<Vec<Card>> {
    let mut cards: Vec<Card> = vec![];
  
    for line in text.lines() { 
        cards.push(line_to_card(line)?);
    }

    Ok(cards)
}

fn line_to_card(line: &str) -> anyhow::Result<Card> {
    let parts: Vec<&str> = line.split(&[':','|'][..]).collect();
    let id_str: &str = parts.first().expect("Missing id").split(' ').collect::<Vec<&str>>().last().expect("Missing id");
    let id: u32 = id_str.parse()?;

    let winning: HashSet<u8> = line_to_numbers(parts.get(1).expect("Missing winning"))?;
    let player: HashSet<u8> = line_to_numbers(parts.get(2).expect("Missing player"))?;

    Ok(Card { id, winning, player })
}

fn line_to_numbers(line: &str) -> anyhow::Result<HashSet<u8>>  {
    let mut numbers: HashSet<u8> = HashSet::new();

    for part in line.split(' ') {
        if !part.is_empty() {
            numbers.insert(part.parse()?);
        }
    }

    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_card() {
        let card = line_to_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").unwrap();
        assert_eq!(card, Card { 
            id: 1, 
            winning: HashSet::from([41, 48, 83, 86, 17]),
            player: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        })
    }
}

