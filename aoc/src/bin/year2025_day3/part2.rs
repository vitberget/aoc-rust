use aoc_procmacros::aoc_profile;

#[aoc_profile]
pub fn part2(text: &str) -> anyhow::Result<usize> {
    let result = text.lines()
        .map(|line| line.trim())
        .map(|line| get_joltage_with_friction(line))
        .map(|joltage| joltage.unwrap())
        .sum();

    Ok(result)
}

fn get_joltage_with_friction(line: &str) -> anyhow::Result<usize> {
    let digits: Vec<usize> = line
        .chars()
        .map(|ch| ch.to_digit(10))
        .map(|option| option.unwrap())
        .map(|digit| digit as usize)
        .collect();

    Ok(get_digit(&digits, 12))    
}

fn get_digit(digits: &[usize], digits_left: usize) -> usize {
    if digits_left == 0 { return 0 }

    let mut bank: Option<(usize, usize)> = None;
    let last_digit_dropped = &digits[0..digits.len() - digits_left + 1];
    for digit in (1_usize..=9).rev() {
        if bank.is_none() {
            if let Some(position) = last_digit_dropped.iter().position(|n| n==&digit) {
                bank = Some((digit, position));
            }
        }
    }

    let bank = bank.unwrap();
    let factor = 10usize.pow(digits_left as u32 - 1);
    bank.0 * factor + get_digit(&digits[bank.1+1..], digits_left-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        let example = include_str!("example.txt");
        let result = part2(example)?;
        assert_eq!(result, 3121910778619);
        Ok(())
    }

    // In 987654321111111, the largest joltage can be found by turning on everything except some 
    // 1s at the end to produce 987654321111.
    #[test]
    fn test_joltage_1() -> anyhow::Result<()> {
        let result = get_joltage_with_friction("987654321111111")?;
        assert_eq!(result, 987654321111);
        Ok(())
    }

    // In the digit sequence 811111111111119, the largest joltage can be found by turning on 
    // everything except some 1s, producing 811111111119.
    #[test]
    fn test_joltage_2() -> anyhow::Result<()> {
        let result = get_joltage_with_friction("811111111111119")?;
        assert_eq!(result, 811111111119);
        Ok(())
    }

    // In 234234234234278, the largest joltage can be found by turning on everything except 
    // a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
    #[test]
    fn test_joltage_3() -> anyhow::Result<()> {
        let result = get_joltage_with_friction("234234234234278")?;
        assert_eq!(result, 434234234278);
        Ok(())
    }

    // In 818181911112111, the joltage 888911112111 is produced by turning on everything except 
    // some 1s near the front.
    #[test]
    fn test_joltage_4() -> anyhow::Result<()> {
        let result = get_joltage_with_friction("818181911112111")?;
        assert_eq!(result, 888911112111);
        Ok(())
    }
}
