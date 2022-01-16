use anyhow::{bail, Context, Result};

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected there to only have 1 line, got {}", lines.len())
    }

    let digits: Option<Vec<u32>> = lines[0].chars().map(|c| c.to_digit(10)).collect();
    let digits = digits.context("could not parse chars as digits")?;

    // Part 1: Sum of all digits that match the next one
    let mut ans1 = 0;
    for ii in 0..digits.len() {
        let (d1, d2) = (digits[ii], digits[(ii + 1) % digits.len()]);
        if d1 == d2 {
            ans1 += d1;
        }
    }

    // Part 2: Sum of digits which matches its "opposite"
    if digits.len() % 2 != 0 {
        bail!("expected digits to be of even length, got {}", digits.len())
    }
    let mut ans2 = 0;
    for ii in 0..(digits.len() / 2) {
        let (d1, d2) = (digits[ii], digits[ii + digits.len() / 2]);
        if d1 == d2 {
            ans2 += d1 * 2;
        }
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
