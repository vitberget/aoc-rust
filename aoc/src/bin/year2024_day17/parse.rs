use anyhow::bail;

use crate::structs::{Computer, Registers};

///Register A: 729
///Register B: 0
///Register C: 0
///
///Program: 0,1,5,4,3,0
pub fn text_to_computer(text: &str) -> anyhow::Result<Computer> {
    const REGISTER_A:&str = "Register A: ";
    const REGISTER_B:&str = "Register B: ";
    const REGISTER_C:&str = "Register C: ";
    const PROGRAM:&str = "Program: ";
    let mut registers = Registers { a: 0, b: 0, c: 0};

    for line in text.lines() {
        if let Some(number_text) = line.strip_prefix(REGISTER_A) { registers.a = number_text.parse()?; }
        if let Some(number_text) = line.strip_prefix(REGISTER_B) { registers.b = number_text.parse()?; }
        if let Some(number_text) = line.strip_prefix(REGISTER_C) { registers.c = number_text.parse()?; }

        if let Some(program_text) = line.strip_prefix(PROGRAM) { 
            let mut program: Vec<u8> = vec![];
            for word in  program_text.split(",") {
                if let Ok(op_code) = word.parse() {
                    program.push(op_code);
                } else {
                    bail!("Failed to parse byte");
                }
            }
            return Ok(Computer { 
                registers, 
                instruction_pointer: 0, 
                program,
                output: vec![]
            });
        }

    }

    bail!("Incorrect text?!?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() -> anyhow::Result<()> {
        let computer = text_to_computer(include_str!("example.txt"))?;

        assert_eq!(computer.registers, Registers { a: 729, b: 0, c:0});
        assert_eq!(computer.program, vec![0,1,5,4,3,0]);
        assert_eq!(computer.instruction_pointer, 0);
        Ok(())
    }
}
