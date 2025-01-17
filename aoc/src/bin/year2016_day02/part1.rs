use std::collections::HashMap;
use std::sync::LazyLock;

use aoc_procmacros::aoc_profile;

static KEYPAD: LazyLock<HashMap<(isize, isize), usize>> = LazyLock::new(||{
    // 1 2 3
    // 4 5 6
    // 7 8 9
    HashMap::from([
        ((0,0), 1), ((1,0), 2), ((2,0), 3),
        ((0,1), 4), ((1,1), 5), ((2,1), 6),
        ((0,2), 7), ((1,2), 8), ((2,2), 9),
    ])
});

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let mut result: usize = 0;
    let mut position: (isize, isize) = (1, 1);

    text.trim()
        .lines()
        .for_each(|line| {
            line.chars()
                .for_each(|ch| {
                    let new_position: (isize, isize) = match ch {
                        'U' => (position.0, position.1 - 1),
                        'D' => (position.0, position.1 + 1),
                        'L' => (position.0 - 1, position.1),
                        'R' => (position.0 + 1, position.1),

                        _ => position,
                    };
                    if KEYPAD.contains_key(&new_position) { position = new_position; }
                });
            result *= 10;
            result += KEYPAD.get(&position).unwrap_or(&0);
        });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part1("ULL\nRRDDD\nLURDL\nUUUUD")?, 1985);
        Ok(())
    }
}
