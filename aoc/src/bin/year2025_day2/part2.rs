pub fn part2(text: &str) -> anyhow::Result<usize> {
    let sum = text.split(',')
        .map(|line| line.trim())
        .flat_map(|line| get_invalid_ids(line).unwrap())
        .sum();

    Ok(sum)
}

fn get_invalid_ids(line: &str) -> anyhow::Result<Vec<usize>> {
    let (low_str, high_str) = line.split_once('-').unwrap();

    let low: usize = low_str.parse()?;
    let high: usize = high_str.parse()?;

    let mut result = vec![];

    for number in low..=high {
        let digits_in_number = number.ilog10() + 1;

        let mut size = 1;
        while size * 2 <= digits_in_number {
            if digits_in_number % size == 0 {
                let id_repeats = break_up_number(number, size)
                    .windows(2)
                    .all(|nums| nums[0]==nums[1]);

                if id_repeats {
                    result.push(number);
                    break;
                }
            }

            size += 1;
        }
    }

    Ok(result)
}

fn break_up_number(number: usize, digits: u32) -> Vec<usize> {
    let mut result = vec![];
    let mut number = number;

    let parter = 10_usize.pow(digits);

    while number > 0 {
        result.push(number % parter);
        number /= parter;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = part2(EXAMPLE)?;
        assert_eq!(result, 4174379265);
        Ok(())
    }

    // 11-22 still has two invalid IDs, 11 and 22.
    #[test]
    fn test_range_1() -> anyhow::Result<()> {
        let result = get_invalid_ids("11-22")?;
        assert_eq!(result, vec![11,22]);
        Ok(())
    }

    // 95-115 now has two invalid IDs, 99 and 111.
    #[test]
    fn test_range_2() -> anyhow::Result<()> {
        let result = get_invalid_ids("95-115")?;
        assert_eq!(result, vec![99,111]);
        Ok(())
    }

    // 998-1012 now has two invalid IDs, 999 and 1010.
    #[test]
    fn test_range_3() -> anyhow::Result<()> {
        let result = get_invalid_ids("998-1012")?;
        assert_eq!(result, vec![999,1010]);
        Ok(())
    }

    // 1188511880-1188511890 still has one invalid ID, 1188511885.
    #[test]
    fn test_range_4() -> anyhow::Result<()> {
        let result = get_invalid_ids("1188511880-1188511890")?;
        assert_eq!(result, vec![1188511885]);
        Ok(())
    }

    // 222220-222224 still has one invalid ID, 222222.
    #[test]
    fn test_range_5() -> anyhow::Result<()> {
        let result = get_invalid_ids("222220-222224")?;
        assert_eq!(result, vec![222222]);
        Ok(())
    }

    // 1698522-1698528 still contains no invalid IDs.
    #[test]
    fn test_range_6() -> anyhow::Result<()> {
        let result = get_invalid_ids("1698522-1698528")?;
        assert!(result.is_empty());
        Ok(())
    }

    // 446443-446449 still has one invalid ID, 446446.
    #[test]
    fn test_range_7() -> anyhow::Result<()> {
        let result = get_invalid_ids("446443-446449")?;
        assert_eq!(result, vec![446446]);
        Ok(())
    }

    // 38593856-38593862 still has one invalid ID, 38593859.
    #[test]
    fn test_range_8() -> anyhow::Result<()> {
        let result = get_invalid_ids("38593856-38593862")?;
        assert_eq!(result, vec![38593859]);
        Ok(())
    }

    // 565653-565659 now has one invalid ID, 565656.
    #[test]
    fn test_range_9() -> anyhow::Result<()> {
        let result = get_invalid_ids("565653-565659")?;
        assert_eq!(result, vec![565656]);
        Ok(())
    }

    // 824824821-824824827 now has one invalid ID, 824824824.
    #[test]
    fn test_range_10() -> anyhow::Result<()> {
        let result = get_invalid_ids("824824821-824824827")?;
        assert_eq!(result, vec![824824824]);
        Ok(())
    }

    // 2121212118-2121212124 now has one invalid ID, 2121212121.
    #[test]
    fn test_range_11() -> anyhow::Result<()> {
        let result = get_invalid_ids("2121212118-2121212124")?;
        assert_eq!(result, vec![2121212121]);
        Ok(())
    }
}
