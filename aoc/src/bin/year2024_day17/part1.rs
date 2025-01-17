use anyhow::bail;
use itertools::Itertools;

use crate::parse::text_to_computer;
use crate::structs::{Computer, OpCode};

pub fn part1(text: &str) -> anyhow::Result<String> {
    let mut computer = text_to_computer(text)?;
    run_program(&mut computer)?;
    Ok(computer.output.iter().join(","))
}

pub fn run_program(computer: &mut Computer) -> anyhow::Result<()> {
    while ! is_halted(computer) {
        tick(computer)?;
    }
    Ok(()) 
}

fn get_combo(computer: &Computer) -> anyhow::Result<u64> {
    match computer.program[computer.instruction_pointer + 1] {
        0 => Ok(0),
        1 => Ok(1),
        2 => Ok(2),
        3 => Ok(3),
        4 => Ok(computer.registers.a),
        5 => Ok(computer.registers.b),
        6 => Ok(computer.registers.c),
        7 => bail!("Reserved combo code"),
        _ => bail!("Outside Octa number!!!")
    }
}

fn get_literal(computer: &Computer) -> u8 {
    computer.program[computer.instruction_pointer + 1]
}

fn tick(computer: &mut Computer) -> anyhow::Result<()> {
    match get_opcode(computer) {
        OpCode::Adv => {
            let combo = get_combo(computer)?;
            computer.registers.a /= 2u64.pow(combo as u32);
            computer.instruction_pointer += 2;
		}
        OpCode::Bxl => {
            let literal = get_literal(computer);
            computer.registers.b ^= literal as u64;
            computer.instruction_pointer += 2;
		}
        OpCode::Bst => {
            let combo = get_combo(computer)?;
            computer.registers.b = combo % 8;
            computer.instruction_pointer += 2;
		}
        OpCode::Jnz => {
            if computer.registers.a != 0 {
                computer.instruction_pointer = get_literal(computer) as usize
            } else {
                computer.instruction_pointer += 2;
            }
		}
        OpCode::Bxc => {
            computer.registers.b ^= computer.registers.c;
            computer.instruction_pointer += 2;
		}
        OpCode::Out => {
            computer.output.push((get_combo(computer)? % 8) as u8);
            computer.instruction_pointer += 2;
		}
        OpCode::Bdv => {
            let combo = get_combo(computer)?;
            computer.registers.b = computer.registers.a / 2u64.pow(combo as u32);
            computer.instruction_pointer += 2;
		}
        OpCode::Cdv => {
            let combo = get_combo(computer)?;
            computer.registers.c = computer.registers.a / 2u64.pow(combo as u32);
            computer.instruction_pointer += 2;
		}
    }
    Ok(())
}

fn get_opcode(computer: &mut Computer) -> OpCode {
    match computer.program[computer.instruction_pointer] {
        0 => OpCode::Adv,
        1 => OpCode::Bxl,
        2 => OpCode::Bst,
        3 => OpCode::Jnz,
        4 => OpCode::Bxc,
        5 => OpCode::Out,
        6 => OpCode::Bdv,
        7 => OpCode::Cdv,
        _ => panic!("Not an opcode")
    }
}

fn is_halted(computer: &Computer) -> bool {
    computer.instruction_pointer >= computer.program.len()
}

#[cfg(test)]
mod tests {
    use crate::structs::Registers;

    use super::*;

    #[test]
    /// If register C contains 9, the program 2,6 would set register B to 1.
    fn test_item_1() -> anyhow::Result<()> {
        let mut computer = Computer {
            registers: Registers { a: 0, b: 0, c: 9 },
            instruction_pointer: 0,
            program: vec![2,6],
            output: vec![]
        };
        tick(&mut computer)?;

        assert_eq!(computer.registers.b, 1);
        assert!(is_halted(&computer));

        Ok(())
    }
    
    #[test]
    /// If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    fn test_item_2() -> anyhow::Result<()> {
        let mut computer = Computer {
            registers: Registers { a: 10, b: 0, c: 0 },
            instruction_pointer: 0,
            program: vec![5,0,5,1,5,4],
            output: vec![]
        };

        run_program(&mut computer)?;

        assert_eq!(computer.output, vec![0,1,2]);

        Ok(())
    }

    #[test]
    /// If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    fn test_item_3() -> anyhow::Result<()> {
        let mut computer = Computer {
            registers: Registers { a: 2024, b: 0, c: 0 },
            instruction_pointer: 0,
            program: vec![0,1,5,4,3,0],
            output: vec![]
        };

        run_program(&mut computer)?;

        assert_eq!(computer.output, vec![4,2,5,6,7,7,7,7,3,1,0]);
        assert_eq!(computer.registers.a, 0);

        Ok(())
    }

    #[test]
    /// If register B contains 29, the program 1,7 would set register B to 26.
    fn test_item_4() -> anyhow::Result<()> {
        let mut computer = Computer {
            registers: Registers { a: 0, b: 29, c: 0 },
            instruction_pointer: 0,
            program: vec![1,7],
            output: vec![]
        };

        run_program(&mut computer)?;

        assert_eq!(computer.registers.b, 26);

        Ok(())
    }

    #[test]
    /// If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    fn test_item_5() -> anyhow::Result<()> {
        let mut computer = Computer {
            registers: Registers { a: 0, b: 2024, c: 43690 },
            instruction_pointer: 0,
            program: vec![4,0],
            output: vec![]
        };

        run_program(&mut computer)?;

        assert_eq!(computer.registers.b, 44354);

        Ok(())
    }


    #[test]
    fn test_example() -> anyhow::Result<()> {
        assert_eq!(part1(include_str!("example.txt"))?, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn test_puzzle() -> anyhow::Result<()> {
        assert_eq!(part1(include_str!("../../../../puzzles/year2024_day17.txt"))?, "1,5,3,0,2,5,2,5,3");
        Ok(())
    }
}
