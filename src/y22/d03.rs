use anyhow::{bail, Context, Result};

fn priority(c: char) -> Result<usize> {
    match c {
        'a'..='z' => Ok((c as usize) - 96),
        'A'..='Z' => Ok((c as usize) - 64 + 26),
        _ => bail!("invalid character {}", c),
    }
}

fn common_item_priority(s1: &str, s2: &str) -> Result<usize> {
    let (mut b1, mut b2) = ([false; 52], [false; 52]);
    for c in s1.chars() {
        b1[priority(c)? - 1] = true;
    }
    for c in s2.chars() {
        b2[priority(c)? - 1] = true;
    }

    for ii in 0..52 {
        if b1[ii] && b2[ii] {
            return Ok(ii + 1);
        }
    }
    bail!("could not find common item")
}

fn common_item_priority_3(s1: &str, s2: &str, s3: &str) -> Result<usize> {
    let (mut b1, mut b2, mut b3) = ([false; 52], [false; 52], [false; 52]);
    for c in s1.chars() {
        b1[priority(c)? - 1] = true;
    }
    for c in s2.chars() {
        b2[priority(c)? - 1] = true;
    }
    for c in s3.chars() {
        b3[priority(c)? - 1] = true;
    }

    for ii in 0..52 {
        if b1[ii] && b2[ii] && b3[ii] {
            return Ok(ii + 1);
        }
    }
    bail!("could not find common item")
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    // Validation for part 2
    if lines.len() == 0 || lines.len() % 3 != 0 {
        bail!("invalid input length: got {}", lines.len())
    }

    // Part 1: Sum of priorities of common items per row
    let ans1 = lines
        .iter()
        .map(|line| {
            let len = line.len();
            if len == 0 || len % 2 != 0 {
                bail!("line {} is of invalid length", line);
            }

            let s1 = &line[..len / 2];
            let s2 = &line[len / 2..];
            common_item_priority(s1, s2)
                .with_context(|| format!("could not get common item for line {}", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not get result for part 1")?
        .iter()
        .sum::<usize>()
        .to_string();

    // Part 2: Sum of priorities by three
    let ans2 = lines
        .chunks(3)
        .into_iter()
        .map(|input| {
            common_item_priority_3(&input[0], &input[1], &input[2])
                .with_context(|| format!("could not get common item for lines {:?}", input))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not get result for part 2")?
        .iter()
        .sum::<usize>()
        .to_string();

    Ok((ans1, ans2))
}
