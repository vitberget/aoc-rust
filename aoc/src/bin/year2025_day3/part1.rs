use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part1(text: &str) -> anyhow::Result<usize> {
    let result = text.lines()
        .map(|line| line.trim())
        .map(|line| get_joltage(line))
        .map(|joltage| joltage.unwrap())
        .sum();

    Ok(result)
}

fn get_joltage(line: &str) -> anyhow::Result<usize> {
    let digits: Vec<usize> = line
        .chars()
        .map(|ch| ch.to_digit(10))
        .map(|option| option.unwrap())
        .map(|digit| digit as usize)
        .collect();
   
    let mut first_bank: Option<(usize, usize)> = None;
    let last_digit_dropped = &digits[0..digits.len()-1];
    for digit in (1_usize..=9).rev() {
        if first_bank.is_none() {
            if let Some(position) = last_digit_dropped.iter().position(|n| n==&digit) {
                first_bank = Some((digit, position));
            }
        }
    }

    let second_bank = digits[first_bank.unwrap().1+1..].iter().max().unwrap();

    Ok(first_bank.unwrap().0 * 10 + second_bank)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let example = include_str!("example.txt");
        let result = part1(example)?;
        assert_eq!(result, 357);
        Ok(())
    }

    // In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
    #[test]
    fn test_joltage_1() -> anyhow::Result<()> {
        let result = get_joltage("987654321111111")?;
        assert_eq!(result, 98);
        Ok(())
    }

    // In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, 
    // producing 89 jolts.
    #[test]
    fn test_joltage_2() -> anyhow::Result<()> {
        let result = get_joltage("811111111111119")?;
        assert_eq!(result, 89);
        Ok(())
    }

    // In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
    #[test]
    fn test_joltage_3() -> anyhow::Result<()> {
        let result = get_joltage("234234234234278")?;
        assert_eq!(result, 78);
        Ok(())
    }

    // In 818181911112111, the largest joltage you can produce is 92.
    #[test]
    fn test_joltage_4() -> anyhow::Result<()> {
        let result = get_joltage("818181911112111")?;
        assert_eq!(result, 92);
        Ok(())
    }
}
