use crate::structs::Game;

pub fn part_1(games: &[Game]) -> anyhow::Result<u32> {
    Ok(games.iter()
       .filter(within_parameters)
       .map(|game| game.id)
       .sum())
}

fn within_parameters(game: &&Game) -> bool {
    game.picks.iter()
        .find(|pick| pick.red > 12 
              || pick.green > 13 
              || pick.blue > 14
             )
        .is_none()
}
