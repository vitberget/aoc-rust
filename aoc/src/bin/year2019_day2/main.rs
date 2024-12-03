use anyhow::bail;


const PUZZLE: &[u32] = &[1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,10,19,23,2,9,23,27,1,6,27,31,1,10,31,35,1,35,10,39,1,9,39,43,1,6,43,47,1,10,47,51,1,6,51,55,2,13,55,59,1,6,59,63,1,10,63,67,2,67,9,71,1,71,5,75,1,13,75,79,2,79,13,83,1,83,9,87,2,10,87,91,2,91,6,95,2,13,95,99,1,10,99,103,2,9,103,107,1,107,5,111,2,9,111,115,1,5,115,119,1,9,119,123,2,123,6,127,1,5,127,131,1,10,131,135,1,135,6,139,1,139,5,143,1,143,9,147,1,5,147,151,1,151,13,155,1,5,155,159,1,2,159,163,1,163,6,0,99,2,0,14,0];

pub fn main() -> anyhow::Result<()> {

    let p1 = solve_part_1(PUZZLE, true)?;
    println!("part1: {p1}");

    let p2 = solve_part_2(PUZZLE)?;
    println!("part2: {p2}");

    Ok(())
}

fn solve_part_1(numbers: &[u32], do_1202: bool) -> anyhow::Result<u32> {
    let mut copy: Box<[u32]> = numbers.to_vec().into_boxed_slice();
    let numbers: &mut [u32] = copy.as_mut();

    if do_1202 {
        numbers[1] = 12;
        numbers[2] = 2;
    }

    for idx in (0..numbers.len()).step_by(4) {
        let opcode = numbers[idx];
        match opcode {
            99 => return Ok(numbers[0]),
            1 => {
                let sum = numbers[numbers[idx + 1] as usize] + numbers[numbers[idx + 2] as usize];
                numbers[numbers[idx + 3] as usize] = sum;
            }
            2 => {
                let product = numbers[numbers[idx + 1] as usize] * numbers[numbers[idx + 2] as usize];
                numbers[numbers[idx + 3] as usize] = product;
            }
            other => bail!("Unknown opcode {other}")
        }
    }


    bail!("Ran outside of instructions")
}

fn solve_part_2(numbers: &[u32]) -> anyhow::Result<u32> {
    for verb in 0..100 {
        for noun in 0..100 {
            if part_2(numbers, noun, verb).unwrap_or(0) == 19690720 {
                return Ok(noun * 100 + verb); 
            }
        }
    }
    bail!("failed to find solution")
}

fn part_2(numbers: &[u32], noun: u32, verb: u32) -> anyhow::Result<u32> {
    let mut copy: Box<[u32]> = numbers.to_vec().into_boxed_slice();
    let numbers: &mut [u32] = copy.as_mut();

    numbers[1] = noun;
    numbers[2] = verb;

    for idx in (0..numbers.len()).step_by(4) {
        let opcode = numbers[idx];
        match opcode {
            99 => return Ok(numbers[0]),
            1 => {
                let sum = numbers[numbers[idx + 1] as usize] + numbers[numbers[idx + 2] as usize];
                numbers[numbers[idx + 3] as usize] = sum;
            }
            2 => {
                let product = numbers[numbers[idx + 1] as usize] * numbers[numbers[idx + 2] as usize];
                numbers[numbers[idx + 3] as usize] = product;
            }
            other => bail!("Unknown opcode {other}")
        }
    }

    bail!("Ran outside of instructions")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &[u32] = &[1,9,10,3,2,3,11,0,99,30,40,50];
    const EXAMPLE_2: &[u32] = &[1,1,1,4,99,5,6,0,99];

    #[test]
    fn test_example() {
        assert_eq!(solve_part_1(EXAMPLE, false).unwrap(), 3500);
        assert_eq!(solve_part_1(EXAMPLE_2, false).unwrap(), 30);
    }
}
