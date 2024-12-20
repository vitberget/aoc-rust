use anyhow::bail;

use crate::{parse::text_to_computer, part1::run_program};

pub fn part2(text: &str) -> anyhow::Result<u64> {
    let mut test_number: u64 = 0;
    loop {
        let mut computer = text_to_computer(text)?;
        computer.registers.a = test_number;
        run_program(&mut computer)?;

        if computer.program == computer.output {
            return Ok(test_number)
        }

        if computer.program.ends_with(&computer.output) {
            // println!("test num {test_number} {test_number:o}\n      {:?}/{} \nfacit {:?}/{}", 
            //     computer.output.iter().rev().collect_vec(), 
            //     computer.output.len(),
            //     computer.program.iter().rev().collect_vec(),
            //     computer.program.len());
            if computer.output.len() > 3 {
                test_number <<= 3;
                test_number -= 1;
            } else {
                test_number += 1;
            }
        } else {
            test_number += 1;
        }

        if test_number > 864860531118056 { bail!("too high")};
        // 864860531118056 -- to high (first try)
    }
}


#[cfg(test)]
mod tests {
    use crate::part2::part2;

    #[test]
    fn test_part2_example() -> anyhow::Result<()> {
        assert_eq!(part2(include_str!("example_part2.txt"))?, 117440);
        Ok(())
    }

    #[test]
    fn test_part2_puzzle() -> anyhow::Result<()> {
        assert_eq!(part2(include_str!("../../../../puzzles/year2024_day17.txt"))?,108107566389757);
        Ok(())
    }
}
