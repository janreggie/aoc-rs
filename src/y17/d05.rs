use crate::util::vectors;
use anyhow::{bail, Context, Result};

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let instructions_master: Vec<i32> =
        vectors::from_strs(&lines).context("could not convert instructions to numbers")?;
    let instructions_len = instructions_master.len() as i32;

    // Part 1: Increment by 1 for each instruction
    let mut instructions = instructions_master.clone();
    let mut step_count = 1;
    let mut ind = 0;
    loop {
        let current = instructions[ind];
        let next_ind = current + (ind as i32);
        if next_ind < 0 || next_ind >= instructions_len {
            break;
        }
        step_count += 1;
        instructions[ind] += 1;
        ind = next_ind as usize;
    }
    let ans1 = step_count.to_string();

    // Part 2: Inc by 1 if <3, dec by 1 otherwise
    let mut instructions = instructions_master.clone();
    let mut step_count = 1;
    let mut ind = 0;
    loop {
        let current = instructions[ind];
        let next_ind = current + (ind as i32);
        if next_ind < 0 || next_ind >= instructions_len {
            break;
        }
        step_count += 1;
        if current >= 3 {
            instructions[ind] -= 1;
        } else {
            instructions[ind] += 1;
        }
        ind = next_ind as usize;
    }
    let ans2 = step_count.to_string();

    Ok((ans1, ans2))
}
