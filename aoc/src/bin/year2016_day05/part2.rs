use anyhow::bail;
use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<String> {

    let mut result: [Option<char>; 8] = [None;8];

    let five_zero_iter = (0..)
        .map(|n| format!("{text}{n}"))
        .map(md5::compute)
        .filter(|digest| digest[0] == 0 && digest[1] == 0 && digest[2] < 0x10);

    for fiver in five_zero_iter {
        let pos: usize = (fiver[2] & 0xF) as usize;
        let char = char::from_digit((fiver[3] >> 4) as u32, 16);

        if pos < 8 {
            if result[pos].is_none() {
                result[pos] = char;
            }

            if result.iter().all(|option| option.is_some()) {
                return Ok(result.iter().flatten().collect());
            }
        }
    }

    bail!("Noooo");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part2("abc")?, "05ace8e3");
        Ok(())
    }
}
