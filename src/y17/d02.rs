use crate::util::vectors;
use anyhow::{bail, Context, Result};

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut spreadsheet: Vec<Vec<u32>> = vec![];
    for line in lines {
        let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let row: Vec<u32> = vectors::from_strs(&row)
            .context(format!("could not convert row {} into numbers", line))?;
        if row.len() == 0 {
            bail!("could not parse empty row")
        }
        spreadsheet.push(row);
    }

    // Part 1: Difference between largest and smallest values
    let mut ans1 = 0;
    for row in &spreadsheet {
        let min = row.iter().min().unwrap();
        let max = row.iter().max().unwrap();
        ans1 += max - min;
    }

    // Part 2: Evenly divisible numbers
    let mut ans2 = 0;
    for row in &spreadsheet {
        // O(n^2) solution is "okay" here since each row has 16 elems.
        // But for now let's think about this through...
        let row_len = row.len();
        let mut done = false;
        for ii in 0..row_len {
            if done {
                break;
            }

            for jj in ii + 1..row_len {
                let (a1, a2) = (row[ii], row[jj]);

                if a1 > a2 && a1 % a2 == 0 {
                    ans2 += a1 / a2;
                    done = true;
                    break;
                } else if a1 < a2 && a2 % a1 == 0 {
                    ans2 += a2 / a1;
                    done = true;
                    break;
                }
            }
        }

        if !done {
            bail!(
                "could not get two numbers that evenly divide in row {:?}",
                row
            );
        }
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
