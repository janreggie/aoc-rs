use anyhow::{Context, Result};
use sscanf::scanf;

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let input: Vec<((u32, u32), (u32, u32))> = lines
        .iter()
        .map(|line| {
            scanf!(line, "{}-{},{}-{}", u32, u32, u32, u32)
                .with_context(|| format!("invalid line {}", line))
                .map(|(a1, a2, b1, b2)| ((a1, a2), (b1, b2)))
        })
        .collect::<Result<_>>()
        .context("invalid input")?;

    // Part 1: Check if one range is "inside" the other
    let mut ans1 = 0;
    for ((a1, a2), (b1, b2)) in &input {
        if (a1 <= b1 && a2 >= b2) || (a1 >= b1 && a2 <= b2) {
            ans1 += 1;
        }
    }
    let ans1 = Ok(ans1.to_string());

    // Part 2: Check if the ranges overlap anywhere
    let mut ans2 = 0;
    for ((a1, a2), (b1, b2)) in &input {
        if b1 <= a2 && a1 <= b2 {
            ans2 += 1;
        }
    }
    let ans2 = Ok(ans2.to_string());

    Ok((ans1, ans2))
}
