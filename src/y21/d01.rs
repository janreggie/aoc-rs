use crate::util::vectors;
use anyhow::{Context, Result};

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let nums = vectors::from_strs::<u32>(&lines)
        .context("could not convert all input into nums")?;

    // Part 1: Larger than previous
    let mut ans1 = 0;
    for ii in 1..nums.len() {
        if nums[ii] > nums[ii - 1] {
            ans1 += 1;
        }
    }
    let ans1 = Ok(ans1.to_string());

    // Part 2: Comparing nums[x] with nums[x+3]
    let mut ans2 = 0;
    for ii in 3..nums.len() {
        if nums[ii] > nums[ii - 3] {
            ans2 += 1;
        }
    }
    let ans2 = Ok(ans2.to_string());

    Ok((ans1, ans2))
}
