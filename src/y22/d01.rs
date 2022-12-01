use anyhow::{Context, Result};

use crate::util::vectors::{from_strs, group};

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut calories: Vec<u32> = group(lines)
        .iter()
        .map(|lines| from_strs::<u32>(lines))
        .collect::<Result<Vec<_>>>()
        .context("could not parse input to u32")?
        .iter()
        .map(|cals| cals.iter().sum())
        .collect();

    // Part 1: Maximum number of calories
    let ans1 = calories
        .iter()
        .max()
        .context("calories are empty")?
        .to_string();

    // Part 2: Top three maximum number of calories
    calories.sort_by(|a, b| b.cmp(a));
    let ans2 = calories.iter().take(3).sum::<u32>().to_string();

    Ok((ans1, ans2))
}
