use std::cmp::max;

use crate::structs::{Game, Pick};

pub fn part_2(games: &[Game]) -> anyhow::Result<u32> {
    Ok(games.iter()
       .map(|game| biggest_pick(&game.picks))
       .map(|pick| power_pick(&pick))
       .sum())
}

fn biggest_pick(picks: &[Pick] ) -> Pick  {
    picks.iter().fold(
        Pick { red: 0, green: 0, blue: 0 }, |pick_a, pick_b| Pick { 
        red: max(pick_a.red, pick_b.red), 
        green: max(pick_a.green, pick_b.green), 
        blue: max(pick_a.blue, pick_b.blue), 
    })
}

fn power_pick(pick: &Pick) -> u32 {
    pick.red * pick.green * pick.blue
}
