use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = "";
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 999999);
        Ok(())
    }
}
