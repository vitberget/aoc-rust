use crate::parse_dirs::{Dir};

fn part_2(dir: Dir) -> u64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::parse_dirs::parse_text;
    use super::*;

    #[test]
    fn test_part1() {
        let parsed_dirs = parse_text(include_str!("../../../examples/year2022_day7.txt"));
        assert_eq!(part_2(parsed_dirs), 1915606);
    }
}