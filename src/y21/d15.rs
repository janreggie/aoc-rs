use anyhow::{bail, Context, Result};
use priority_queue::DoublePriorityQueue;
use std::collections::HashSet;

struct Cavern {
    risks: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Cavern {
    fn new(lines: Vec<String>) -> Result<Cavern> {
        let height = lines.len();
        let width = lines.first().unwrap_or(&String::new()).len();
        if height == 0 || width == 0 {
            bail!("input cannot be empty");
        }

        let mut risks = Vec::new();
        for line in lines {
            let row: Option<Vec<u32>> = line.chars().map(|x| x.to_digit(10)).collect();
            let row = row.context(format!("could not format `{}` as a row of numbers", line))?;
            if row.len() != width {
                bail!(
                    "expected row `{}` to be of length {}, got {} instead",
                    line,
                    width,
                    row.len()
                );
            }
            risks.push(row);
        }

        Ok(Cavern {
            risks,
            width,
            height,
        })
    }

    fn least_risk(&self) -> u32 {
        let mut total_risks = vec![vec![None; self.width]; self.height];
        total_risks[0][0] = Some(0);
        let mut pq = DoublePriorityQueue::new();
        pq.push((0, 0), 0);
        let mut done = HashSet::new();
        while let Some(((x, y), risk)) = pq.pop_min() {
            if x == self.width - 1 && y == self.height - 1 {
                return risk;
            }

            // Append neighbors
            if x > 1 && !done.contains(&(x - 1, y)) {
                pq.push_decrease((x - 1, y), risk + self.risks[y][x - 1]);
            }
            if x < self.width - 1 && !done.contains(&(x + 1, y)) {
                pq.push_decrease((x + 1, y), risk + self.risks[y][x + 1]);
            }
            if y > 1 && !done.contains(&(x, y - 1)) {
                pq.push_decrease((x, y - 1), risk + self.risks[y - 1][x]);
            }
            if y < self.height - 1 && !done.contains(&(x, y + 1)) {
                pq.push_decrease((x, y + 1), risk + self.risks[y + 1][x]);
            }

            // Finally, add popint to the done pile
            done.insert((x, y));
        }

        // If we're left with nothing...
        0
    }

    /// Part 2: Expand the cavern
    fn expand(&self) -> Cavern {
        let orig_width = self.width;
        let orig_height = self.height;

        let width = orig_width * 5;
        let height = orig_height * 5;
        let mut risks = vec![vec![0; width]; height];

        for i in 0..5 {
            for j in 0..5 {
                let m = (i + j) as u32;
                let (x_off, y_off) = (orig_width * i, orig_height * j);
                for x in 0..orig_width {
                    for y in 0..orig_height {
                        let o = self.risks[y][x];
                        risks[y_off + y][x_off + x] = (o + m - 1) % 9 + 1;
                    }
                }
            }
        }

        Cavern {
            risks,
            width,
            height,
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let cavern = Cavern::new(lines).context("could not parse input")?;

    let ans1 = cavern.least_risk();
    let cavern = cavern.expand();
    let ans2 = cavern.least_risk();

    Ok((ans1.to_string(), ans2.to_string()))
}
