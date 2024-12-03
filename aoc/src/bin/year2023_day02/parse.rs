use anyhow::{bail, Context};

use crate::structs::{Game, Pick};

pub fn parse_to_games(text: String) -> anyhow::Result<Vec<Game>> {
    let mut result: Vec<Game> = vec![];

    for line in text.lines() {
        result.push(line_to_game(line)?);
    }

    Ok(result)
}

fn line_to_game(line: &str) -> anyhow::Result<Game> {
    let mut parts = line.split(':');
    let id: u32 = line_to_id(parts.next())?;
    let picks: Vec<Pick> = line_to_picks(parts.next())?;

    Ok(Game { id, picks })
}

fn line_to_id(line: Option<&str>) -> anyhow::Result<u32> {
    Ok(line.context("Missing id part")?
        .split(' ')
        .last().context("Missing id part")?
        .trim()
        .parse()?)
}

fn line_to_picks(line: Option<&str>) -> anyhow::Result<Vec<Pick>> {
    let text = line.context("Missing picks")?;
    let mut result: Vec<Pick> = vec![];
    for pick_text in text.split(';') { 
        result.push(text_to_pick(pick_text)?); 
    }
    Ok(result)
}

fn text_to_pick(text: &str) -> anyhow::Result<Pick> {
    let mut red: u32 = 0;
    let mut green: u32 = 0;
    let mut blue: u32 = 0;

    for big_part in text.split(',') {
        let mut parts = big_part.trim().split(' ');
        if let Some(amount) = parts.next() {
            let amount: u32 = amount.trim().parse()?;

            if let Some(color) = parts.next() {
                match color.trim() {
                    "red" => red = amount, 
                    "green" => green = amount, 
                    "blue" => blue = amount, 
                    col => bail!("unknown color {col}")
                }
            }
        }
    }

    Ok(Pick { red, green, blue })
}


#[cfg(test)]
mod tests {
    use crate::structs::Pick;

    use super::*;

    #[test]
    fn test_parse_game() {
        assert_eq!(
            line_to_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap(),
            Game {
                id:1, 
                picks: vec![
                    Pick { red: 4, green: 0, blue: 3 },
                    Pick { red: 1, green: 2, blue: 6 },
                    Pick { red: 0, green: 2, blue: 0 },
                ]});
    }
}
