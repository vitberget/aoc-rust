pub fn part1(text: &str) -> anyhow::Result<usize> {
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

        if digits_in_number % 2 == 0 {
            let divider = 10_usize.pow(digits_in_number / 2);
            let left = number / divider;
            let right = number % divider;
            if left == right {
                result.push(number);
            }
        } 
    }

    Ok(result)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> anyhow::Result<()> {
        const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = part1(EXAMPLE)?;
        assert_eq!(result, 1227775554); 
        Ok(())
    }

    

    /// 11-22 has two invalid IDs, 11 and 22.
    #[test]
    fn test_range_11_22() -> anyhow::Result<()> {
        let result = get_invalid_ids("11-22")?;
        assert_eq!(result, vec![11,22]);
        Ok(())
    }

    // 95-115 has one invalid ID, 99.
    #[test]
    fn test_range_95_115() -> anyhow::Result<()> {
        let result = get_invalid_ids("95-115")?;
        assert_eq!(result, vec![99]);
        Ok(())
    }

    // 998-1012 has one invalid ID, 1010.
    #[test]
    fn test_range_998_1012() -> anyhow::Result<()> {
        let result = get_invalid_ids("998-1012")?;
        assert_eq!(result, vec![1010]);
        Ok(())
    }

    // 1188511880-1188511890 has one invalid ID, 1188511885.
    #[test]
    fn test_range_big() -> anyhow::Result<()> {
        let result = get_invalid_ids("1188511880-1188511890")?;
        assert_eq!(result, vec![1188511885]);
        Ok(())
    }

    // 222220-222224 has one invalid ID, 222222.
    #[test]
    fn test_range_semi_big_1() -> anyhow::Result<()> {
        let result = get_invalid_ids("222220-222224")?;
        assert_eq!(result, vec![222222]);
        Ok(())
    }

    // 1698522-1698528 contains no invalid IDs.
    #[test]
    fn test_range_semi_big_2() -> anyhow::Result<()> {
        let result = get_invalid_ids("1698522-1698528")?;
        assert!(result.is_empty());
        Ok(())
    }

    // 446443-446449 has one invalid ID, 446446.
    #[test]
    fn test_range_semi_big_3() -> anyhow::Result<()> {
        let result = get_invalid_ids("446443-446449")?;
        assert_eq!(result, vec![446446]);
        Ok(())
    }

    // 38593856-38593862 has one invalid ID, 38593859.
    #[test]
    fn test_range_big_2() -> anyhow::Result<()> {
        let result = get_invalid_ids("38593856-38593862")?;
        assert_eq!(result, vec![38593859]);
        Ok(())
    }

    // The rest of the ranges contain no invalid IDs.
    #[test]
    fn test_no_ids() -> anyhow::Result<()> {
        let result = get_invalid_ids("565653-565659")?;
        assert!(result.is_empty());
        let result = get_invalid_ids("824824821-824824827")?;
        assert!(result.is_empty());
        let result = get_invalid_ids("2121212118-2121212124")?;
        assert!(result.is_empty());

        Ok(())
    }

}
