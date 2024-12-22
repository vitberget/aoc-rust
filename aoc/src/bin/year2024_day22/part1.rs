use aoc_procmacros::aoc_profile;
use separator::usize;

#[aoc_profile]
pub fn solve_part_1(text: &str) -> anyhow::Result<u64> {
    Ok(text.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|number| evolve_secret_number_multiple(&number, 2000))
        .sum())
}

pub fn evolve_secret_number_multiple(number: &u64, times: usize) -> u64 {
    (0..times).fold(*number, |number, _| evolve_secret_number(&number))
}

pub fn evolve_secret_number(number: &u64) -> u64 {
    let mut new_number:u64  = *number;  

    new_number ^= new_number << 6;
    new_number &= 0xFFFFFF;

    new_number ^= new_number >> 5;
    new_number &= 0xFFFFFF;

    new_number ^= new_number << 11;
    new_number &= 0xFFFFFF;

    new_number
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;


    #[test]
    fn example() -> anyhow::Result<()> {
        let example = "1\n10\n100\n2024";
        assert_eq!(solve_part_1(example)?, 37327623);
        Ok(())
    }

    #[test]
    fn test_2000_steps() {
        assert_eq!( evolve_secret_number_multiple(&1, 2000), 8685429);
        assert_eq!( evolve_secret_number_multiple(&10, 2000), 4700978);
        assert_eq!( evolve_secret_number_multiple(&100, 2000), 15273692);
        assert_eq!( evolve_secret_number_multiple(&2024, 2000), 8667524);
    }

    #[test]
    fn test_ten_steps_magic_number() {
        let numbers: Vec<u64> = vec![
            123,
            15887950,
            16495136,
            527345,
            704524,
            1553684,
            12683156,
            11100544,
            12249484,
            7753432,
            5908254,
        ];

        for (before, after) in numbers.iter().tuple_windows() {
            println!("{before} should become {after}");
            assert_eq!(evolve_secret_number(before), *after);

        }
    }

    #[test]
    fn part_of_calc() {
        let mut number:u64 = 100000000;
        number %= 16777216;
        assert_eq!(number, 16113920);

        let mut number: u64 = 42;
        number ^= 15;
        assert_eq!(number, 37);
    }
}
