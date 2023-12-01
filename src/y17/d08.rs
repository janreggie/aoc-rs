use std::collections::HashMap;

use anyhow::{bail, Context, Ok, Result};
use sscanf::scanf;

struct CPU {
    registers: HashMap<String, i32>,
}

impl CPU {
    fn new() -> CPU {
        CPU { registers: HashMap::new() }
    }

    fn run_instruction(&mut self, instruction: Instruction) {
        if self.check_condition(&instruction.condition) {
            let &value = self.registers.get(&instruction.operand).unwrap_or(&0);
            self.registers
                .insert(instruction.operand, value + instruction.increase_by);
        }
    }

    fn check_condition(&self, condition: &Condition) -> bool {
        let register_value = *self.registers.get(&condition.lhs).unwrap_or(&0);
        match condition.sign {
            Relation::GT => register_value > condition.rhs,
            Relation::LT => register_value < condition.rhs,
            Relation::EQ => register_value == condition.rhs,
            Relation::GE => register_value >= condition.rhs,
            Relation::LE => register_value <= condition.rhs,
            Relation::NE => register_value != condition.rhs,
        }
    }

    // Will either return the highest valued register, or none if there are no registers.
    fn highest_register(&self) -> Option<(&String, i32)> {
        if let Some((k, v)) = self.registers.iter().max_by_key(|(_k, &v)| v) {
            Some((k, *v))
        } else {
            None
        }
    }
}

struct Instruction {
    operand: String,
    increase_by: i32, // negative if `dec` operation
    condition: Condition,
}

impl Instruction {
    fn new(input: &str) -> Result<Instruction> {
        let (operand, operation, addend, condition) =
            scanf!(input, "{} {} {} if {}", String, String, i32, String)
                .context("could not parse instruction")?;
        let increase_by = match operation.as_str() {
            "inc" => addend,
            "dec" => -addend,
            _ => bail!("invalid operation {}", operation),
        };
        let condition = Condition::new(&condition).with_context(|| {
            format!("could not format string '{}' as condition", condition)
        })?;
        Ok(Instruction { operand, increase_by, condition })
    }
}

struct Condition {
    lhs: String,
    sign: Relation,
    rhs: i32,
}

impl Condition {
    fn new(input: &str) -> Result<Condition> {
        let (lhs, sign, rhs) = scanf!(input, "{} {} {}", String, String, i32)
            .context("could not parse condition")?;
        let sign = Relation::new(&sign)
            .with_context(|| format!("invalid sign representation {}", sign))?;
        Ok(Condition { lhs, sign, rhs })
    }
}

enum Relation {
    GT,
    LT,
    EQ,
    GE,
    LE,
    NE,
}

impl Relation {
    fn new(input: &str) -> Result<Relation> {
        match input {
            ">" => Ok(Relation::GT),
            "<" => Ok(Relation::LT),
            "==" => Ok(Relation::EQ),
            ">=" => Ok(Relation::GE),
            "<=" => Ok(Relation::LE),
            "!=" => Ok(Relation::NE),
            _ => bail!("unknown input {}", input),
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let instructions = lines
        .iter()
        .map(|s| {
            Instruction::new(s)
                .with_context(|| format!("could not parse instruction '{}'", s))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not read input properly")?;
    let mut cpu = CPU::new();

    // Part 1: Run all instructions and get the largest value.
    // Part 2: While running all instructions, get the register with the highest value.
    let mut record_value = i32::MIN;
    for instr in instructions {
        cpu.run_instruction(instr);
        if let Some((_, max_value)) = cpu.highest_register() {
            record_value = record_value.max(max_value);
        }
    }
    let ans1 = Ok(cpu
        .highest_register()
        .context(
            "CPU has no registers for some reason (this shouldn't happen!)",
        )?
        .1
        .to_string());
    let ans2 = Ok(record_value.to_string());

    Ok((ans1, ans2))
}
