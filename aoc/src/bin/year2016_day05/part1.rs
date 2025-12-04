use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<String> {

    let result: String = (0..)
        .map(|n| format!("{text}{n}"))
        .map(md5::compute)
        .filter(|digest| digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10)
        .take(8)
        .map(|digest| format!("{:016x}", digest))
        .flat_map(|str| str.chars().nth(5))
        .collect();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part1("abc")?, "18f47a30");    
        Ok(())
    }

    #[test]
    fn test_format() {
        assert_eq!(format!("{:04x}", 1), "0001");
    }
}
