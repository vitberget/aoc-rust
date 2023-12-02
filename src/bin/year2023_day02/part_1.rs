use crate::structs::Game;

pub fn part_1(games: &[Game]) -> anyhow::Result<u32> {
    Ok(games.iter()
       .filter(within_parameters)
       .map(|game| game.id)
       .sum())
}

fn within_parameters(game: &&Game) -> bool {
    for pick in &game.picks {
        if pick.red > 12 { return false }
        if pick.green > 13 { return false }
        if pick.blue > 14 { return false }
    }
    true
}
