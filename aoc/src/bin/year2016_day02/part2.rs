use std::collections::HashMap;
use std::sync::LazyLock;

use aoc_procmacros::aoc_profile;

static KEYPAD: LazyLock<HashMap<(isize, isize), char>> = LazyLock::new(||{
    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    HashMap::from([
        ((2, 0), '1'),
        ((1, 1), '2'), ((2, 1), '3'), ((3, 1), '4'),
        ((0, 2), '5'), ((1, 2), '6'), ((2, 2), '7'), ((3, 2), '8'), ((4, 2), '9'), 
        ((1, 3), 'A'), ((2, 3), 'B'), ((3, 3), 'C'),
        ((2, 4), 'D'),
    ])
});

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<String> {
    let mut result: String = String::new();
    let mut position: (isize, isize) = (0, 2);

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
            let kp = KEYPAD.get(&position).unwrap_or(&' ');
            result = format!("{result}{kp}");
        });
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part2("ULL\nRRDDD\nLURDL\nUUUUD")?, "5DB3");
        Ok(())
    }
}
