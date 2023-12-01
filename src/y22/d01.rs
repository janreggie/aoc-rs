use anyhow::{bail, Context, Result};

use crate::util::vectors::{from_strs, group};

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let calories: Vec<u32> = group(lines)
        .iter()
        .map(|lines| from_strs::<u32>(lines))
        .collect::<Result<Vec<_>>>()
        .context("could not parse input to u32")?
        .iter()
        .map(|cals| cals.iter().sum())
        .collect();
    if calories.len() < 3 {
        bail!("input too short")
    }

    // Part 1: Maximum number of calories
    let ans1 = Ok(calories.iter().max().unwrap().to_string());

    // Part 2: Top three maximum number of calories
    let mut highest = [calories[0], calories[1], calories[2]];
    highest.sort();
    for &vv in &calories {
        if vv > highest[0] {
            highest[0] = vv;
            highest.sort();
        }
    }
    let ans2 = Ok(highest.iter().sum::<u32>().to_string());

    Ok((ans1, ans2))
}
