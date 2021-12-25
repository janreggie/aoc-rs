use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;

struct ALU {
    instructions: Vec<Instruction>,
}

impl ALU {
    fn new(lines: Vec<String>) -> Result<ALU> {
        let mut instructions = Vec::new();
        for line in lines {
            match Instruction::new(&line) {
                Some(instr) => instructions.push(instr),
                None => bail!("could not parse instruction `{}`", line),
            }
        }
        Ok(ALU { instructions })
    }

    /// Will always reutrn False if there aren't 14 Inp's in the ALU
    fn run_monad(&self, inputs: [i128; 14]) -> bool {
        let mut ind = 0; // index for inputs
        let mut regs = HashMap::new();
        regs.insert(Reg::W, 0);
        regs.insert(Reg::X, 0);
        regs.insert(Reg::Y, 0);
        regs.insert(Reg::Z, 0);

        fn get_r_a(reg: &Reg, arg: &Arg, regs: &HashMap<Reg, i128>) -> (i128, i128) {
            let ref_r = regs.get(reg).unwrap();
            let ref_a = match arg {
                Arg::Literal(v) => v,
                Arg::Reference(r) => regs.get(r).unwrap(),
            };
            (*ref_r, *ref_a)
        }

        let mut interpret = |instr: &Instruction| match instr {
            Instruction::Inp(r) => {
                let rf = regs.get_mut(r).unwrap();
                *rf = inputs[ind];
                ind += 1;
            }
            Instruction::Add(r, a) => {
                let (rv, av) = get_r_a(r, a, &regs);
                let ref_r = regs.get_mut(r).unwrap();
                *ref_r = rv + av;
            }
            Instruction::Mul(r, a) => {
                let (rv, av) = get_r_a(r, a, &regs);
                let ref_r = regs.get_mut(r).unwrap();
                *ref_r = rv * av;
            }
            Instruction::Div(r, a) => {
                let (rv, av) = get_r_a(r, a, &regs);
                let ref_r = regs.get_mut(r).unwrap();
                *ref_r = rv / av;
            }
            Instruction::Mod(r, a) => {
                let (rv, av) = get_r_a(r, a, &regs);
                let ref_r = regs.get_mut(r).unwrap();
                *ref_r = rv % av;
            }
            Instruction::Eql(r, a) => {
                let (rv, av) = get_r_a(r, a, &regs);
                let ref_r = regs.get_mut(r).unwrap();
                *ref_r = if rv == av { 1 } else { 0 };
            }
        };

        for instr in &self.instructions {
            interpret(instr);
        }

        *regs.get(&Reg::Z).unwrap() == 0
    }

    /// Will count the number of Inp's in the ALU
    fn count_inp(&self) -> usize {
        let mut result = 0;
        for instr in &self.instructions {
            if let Instruction::Inp(_) = instr {
                result += 1;
            }
        }
        result
    }
}

enum Instruction {
    Inp(Reg),
    Add(Reg, Arg),
    Mul(Reg, Arg),
    Div(Reg, Arg),
    Mod(Reg, Arg),
    Eql(Reg, Arg),
}

impl Instruction {
    fn new(input: &str) -> Option<Instruction> {
        use Instruction::*;
        let symbols = vectors::split_and_trim(input, ' ');
        let len = symbols.len();
        if len != 2 && len != 3 {
            return None;
        }

        match (symbols[0].as_str(), len) {
            ("inp", 2) => Some(Inp(Reg::new(&symbols[1])?)),
            ("add", 3) => Some(Add(Reg::new(&symbols[1])?, Arg::new(&symbols[2])?)),
            ("mul", 3) => Some(Mul(Reg::new(&symbols[1])?, Arg::new(&symbols[2])?)),
            ("div", 3) => Some(Div(Reg::new(&symbols[1])?, Arg::new(&symbols[2])?)),
            ("mod", 3) => Some(Mod(Reg::new(&symbols[1])?, Arg::new(&symbols[2])?)),
            ("eql", 3) => Some(Eql(Reg::new(&symbols[1])?, Arg::new(&symbols[2])?)),
            (_, _) => None,
        }
    }
}

enum Arg {
    Literal(i128),
    Reference(Reg),
}

impl Arg {
    fn new(input: &str) -> Option<Arg> {
        if let Ok(x) = input.parse::<i128>() {
            Some(Arg::Literal(x))
        } else {
            Some(Arg::Reference(Reg::new(input)?))
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

impl Reg {
    fn new(input: &str) -> Option<Reg> {
        match input {
            "w" => Some(Reg::W),
            "x" => Some(Reg::X),
            "y" => Some(Reg::Y),
            "z" => Some(Reg::Z),
            _ => None,
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let alu = ALU::new(lines).context("could not create alu")?;

    let mut digits = [9; 14];
    fn merge_digits(digits: [i128; 14]) -> i128 {
        let mut result = 0;
        for ii in 0..14 {
            result *= 10;
            result += digits[ii] as i128;
        }
        result
    }
    fn sub_one(mut digits: [i128; 14]) -> [i128; 14] {
        let mut ind = 13;
        digits[ind] -= 1;
        while digits[ind] == 0 {
            digits[ind] = 9;
            ind -= 1;
            digits[ind] -= 1;
        }
        digits
    }

    // Part 1: I don't know really.
    // TODO: Use expressions!
    let ans1;
    loop {
        if digits[11] == 9 && digits[12] == 9 && digits[13] == 9 {
            eprintln!("{}", merge_digits(digits));
        }
        if alu.run_monad(digits) {
            ans1 = merge_digits(digits);
            break;
        }
        digits = sub_one(digits);
    }

    Ok((ans1.to_string(), String::from("unimplemented")))
}
