use aoc_procmacros::aoc_profile;
use aoc_utils::char_map::CharMap;
use aoc_utils::position::Position;

use crate::part1::find_race_track;

#[aoc_profile]
pub fn part_2(text: &str, filter_value: usize, manhattan: isize) -> anyhow::Result<usize> {
    let char_map: CharMap = text.into();

    let race_track = find_race_track(&char_map);

    Ok(find_wall_holes(&race_track, manhattan).iter()
        .filter(|n| **n>=filter_value)
        .count())
}

fn find_wall_holes(race_track: &[Position], manhattan: isize) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    for (start_idx, start_pos) in race_track.iter().enumerate() {
        for (end_idx, end_pos) in race_track.iter().enumerate() {
            if end_idx > start_idx {
                let man = (*end_pos - *start_pos).manhattan();
                if man <= manhattan && (end_idx - start_idx) > man as usize {
                    let r = end_idx - start_idx - man as usize;
                    result.push(r);
                }
            }
        }
    } 
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore]
    fn test_example_p2() -> anyhow::Result<()> {
        let result = part_2(include_str!("example.txt"), 50, 20)?;
        let facit =  32 + 31 + 29 + 39 + 25 + 23 + 20 + 19 + 12 + 14 + 12 + 22 + 4  + 3;

        assert_eq!(result, facit);

        // There are 32 cheats that save 50 picoseconds.
        // There are 31 cheats that save 52 picoseconds.
        // There are 29 cheats that save 54 picoseconds.
        // There are 39 cheats that save 56 picoseconds.
        // There are 25 cheats that save 58 picoseconds.
        // There are 23 cheats that save 60 picoseconds.
        // There are 20 cheats that save 62 picoseconds.
        // There are 19 cheats that save 64 picoseconds.
        // There are 12 cheats that save 66 picoseconds.
        // There are 14 cheats that save 68 picoseconds.
        // There are 12 cheats that save 70 picoseconds.
        // There are 22 cheats that save 72 picoseconds.
        // There are 4 cheats that save 74 picoseconds.
        // There are 3 cheats that save 76 picoseconds.

        Ok(())
    }

    #[test]
    fn test_example_p2_though_p1() -> anyhow::Result<()> {
        let result = part_2(include_str!("example.txt"), 0, 2)?;
        let facit = 14 + 14 + 2  + 4  + 2  + 3  + 1 + 1 + 1 + 1 + 1;

        assert_eq!(result, facit);

        // There are 14 cheats that save 2 picoseconds.
        // There are 14 cheats that save 4 picoseconds.
        // There are 2 cheats that save 6 picoseconds.
        // There are 4 cheats that save 8 picoseconds.
        // There are 2 cheats that save 10 picoseconds.
        // There are 3 cheats that save 12 picoseconds.
        // There is one cheat that saves 20 picoseconds.
        // There is one cheat that saves 36 picoseconds.
        // There is one cheat that saves 38 picoseconds.
        // There is one cheat that saves 40 picoseconds.
        // There is one cheat that saves 64 picoseconds.

        Ok(())
    }
}
