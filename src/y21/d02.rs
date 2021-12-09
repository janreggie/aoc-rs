use crate::util::vectors;
use anyhow::{bail, Context, Result};

/// A submarine instruction.
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Instruction {
    /// Create insturction from line such as `forward 5` or `down 4`
    fn new(line: &str) -> Result<Instruction> {
        let split = vectors::split_and_trim(line, ' ');
        if split.len() != 2 {
            bail!(
                "expected line to be split into two parts, got {}",
                split.len()
            );
        }

        let count: u32 = split[1]
            .parse()
            .context(format!("could not format count `{}` as integer", split[1]))?;

        match split[0].as_str() {
            "forward" => Ok(Instruction::Forward(count)),
            "down" => Ok(Instruction::Down(count)),
            "up" => Ok(Instruction::Up(count)),
            _ => bail!("could not interpret instruction `{}` properly", split[0]),
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut instrs = Vec::new();
    for line in lines {
        let instr = Instruction::new(&line)
            .context(format!("could not parse instruction of line `{}`", line))?;
        instrs.push(instr);
    }

    // Part 1: A simpler way to go through all instructions
    let mut horizontal: u32 = 0;
    let mut vertical: u32 = 0;
    for instr in &instrs {
        match instr {
            Instruction::Forward(c) => horizontal += c,
            Instruction::Down(c) => vertical += c,
            Instruction::Up(c) => vertical -= c,
        }
    }
    let ans1 = horizontal * vertical;

    // Part 2: Something about "aim"
    let mut horizontal: u32 = 0;
    let mut vertical: u32 = 0;
    let mut aim: u32 = 0;
    for instr in &instrs {
        match instr {
            Instruction::Forward(c) => {
                horizontal += c;
                vertical += aim * c;
            }
            Instruction::Down(c) => aim += c,
            Instruction::Up(c) => aim -= c,
        }
    }
    let ans2 = horizontal * vertical;

    Ok((ans1.to_string(), ans2.to_string()))
}
