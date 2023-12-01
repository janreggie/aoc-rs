use anyhow::{Context, Result};

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let calibration_values: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();
    let first_and_last = calibration_values
        .iter()
        .map(|vals| {
            vals.first().and_then(|v1| vals.last().map(|v2| v1 * 10 + v2))
        })
        .collect::<Option<Vec<u32>>>()
        .context("some input does not contain values")?;
    let ans1 = first_and_last.iter().sum::<u32>().to_string();

    Ok((ans1, "unimplemented".to_string()))
}
