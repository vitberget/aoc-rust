use std::collections::HashSet;

use aoc_utils::char_map::{text_to_char_map, CharMap, Position};

pub fn part_1(text: &str, filter_value: usize) -> anyhow::Result<usize> {
    let char_map = text_to_char_map(text);

    let race_track = find_race_track(&char_map);

    let cheats = find_wall_holes(&race_track, &char_map);
    let cheats:usize = cheats.iter()
        .map(|(p1,p2)| (
                race_track.iter().position(|p| p == p1).unwrap(),
                race_track.iter().position(|p| p == p2).unwrap()))
        .filter(|(_,p2)| *p2 > 2)
        .map(|(p1,p2)| (p1, p2-2))
        .filter(|(p1,p2)| p2 > p1)
        .map(|(p1,p2)| p2 - p1 )
        .filter(|n| *n >= filter_value)
        .count();

    Ok(cheats)
}

fn find_wall_holes(race_track: &[Position], char_map: &CharMap) -> Vec<(Position, Position)> {
    let walls = char_map.get(&'#').unwrap();
    let mut result: Vec<(Position, Position)> = vec![];
    for pos in race_track {
        let p1 = *pos + Position::new(1,0);
        let p2 = *pos + Position::new(2,0);
        if walls.contains(&p1) && race_track.contains(&p2) {
            result.push((*pos, p2));
        }
        let p1 = *pos + Position::new(-1,0);
        let p2 = *pos + Position::new(-2,0);
        if walls.contains(&p1) && race_track.contains(&p2) {
            result.push((*pos, p2));
        }
        let p1 = *pos + Position::new(0,1);
        let p2 = *pos + Position::new(0,2);
        if walls.contains(&p1) && race_track.contains(&p2) {
            result.push((*pos, p2));
        }
        let p1 = *pos + Position::new(0,-1);
        let p2 = *pos + Position::new(0,-2);
        if walls.contains(&p1) && race_track.contains(&p2) {
            result.push((*pos, p2));
        }
    } 
    result
}

fn find_race_track(char_map: &CharMap) -> Vec<Position> {
    let mut current_pos: Position = *char_map.get(&'S').unwrap().iter().next().unwrap();
    let end_pos: Position = *char_map.get(&'E').unwrap().iter().next().unwrap();
    let mut race_track: Vec<Position> = vec![current_pos];


    let track = char_map.get(&'.').unwrap();

    loop {
        if current_pos == end_pos { break; }
        current_pos = next_pos(&current_pos, track, &race_track, &end_pos);
        race_track.push(current_pos);
    }



    race_track
}

fn next_pos(current_pos: &Position, track: &HashSet<Position>, race_track: &[Position], end_pos: &Position) -> Position {
    for pos in [Position::new(-1,0), Position::new(1,0), Position::new(0,-1), Position::new(0,1)] {
        let next_pos = *current_pos + pos;
        if (next_pos == *end_pos || track.contains(&next_pos)) && !race_track.contains(&next_pos) {
            return next_pos;
        }
    }
    panic!("No track!");
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let _result = part_1(include_str!("example.txt"), 0)?;
        Ok(())
    }
}
