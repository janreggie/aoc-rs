use crate::util::vectors;
use anyhow::{Context, Result};

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let instructions_master: Vec<i32> = vectors::from_strs(&lines)
        .context("could not convert instructions to numbers")?;

    // Part 1: Increment by 1 for each instruction
    let mut instructions = instructions_master.clone();
    let mut step_count = 0;
    let mut ind = 0;
    while let Some(current) = instructions.get_mut(ind as usize) {
        ind += *current;
        step_count += 1;
        *current += 1;
    }
    let ans1 = Ok(step_count.to_string());

    // Part 2: Inc by 1 if <3, dec by 1 otherwise
    let mut instructions = instructions_master.clone();
    let mut step_count = 0;
    let mut ind = 0;
    while let Some(current) = instructions.get_mut(ind as usize) {
        ind += *current;
        step_count += 1;
        if *current >= 3 {
            *current -= 1;
        } else {
            *current += 1;
        }
    }
    let ans2 = Ok(step_count.to_string());

    Ok((ans1, ans2))
}
