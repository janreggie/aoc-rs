use crate::util::vectors;
use anyhow::{bail, Context, Result};

#[derive(Debug)]
struct LanternfishCounts {
    day: usize,
    counts: [u128; 9],
}

impl LanternfishCounts {
    fn new(fish: &Vec<u32>) -> Result<LanternfishCounts> {
        let mut result = LanternfishCounts {
            day: 0,
            counts: [0; 9],
        };
        for ff in fish {
            if *ff <= 8 {
                result.counts[*ff as usize] += 1;
            } else {
                bail!("invalid timer {}", ff)
            }
        }

        Ok(result)
    }

    /// Progress by one day
    fn next(&mut self) {
        let to_breed = self.counts[0] as u128; // How many Lanternfish will bear children this day?
        for ii in 0..8 {
            self.counts[ii] = self.counts[ii + 1];
        }

        self.counts[6] += to_breed;
        self.counts[8] = to_breed;
        self.day += 1;
    }

    /// Count the number of Lanternfish
    fn count(&self) -> u128 {
        let mut result = 0;
        for c in self.counts {
            result += c;
        }
        result
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected number of lines to be 1, got {}", lines.len());
    }

    let init_counts = &lines[0];
    let init_counts = vectors::split_and_trim(init_counts, ',');
    let init_counts: Vec<u32> =
        vectors::from_strs(&init_counts).context("could not interpret input line")?;

    let mut lanternfish_counts =
        LanternfishCounts::new(&init_counts).context("could not create LanternfishCounts")?;

    // Part 2: Up to 80 days
    while lanternfish_counts.day < 80 {
        lanternfish_counts.next();
    }
    let ans1 = lanternfish_counts.count();

    // Part 2: Up to 256 days
    while lanternfish_counts.day < 256 {
        lanternfish_counts.next();
    }
    let ans2 = lanternfish_counts.count();

    Ok((ans1.to_string(), ans2.to_string()))
}
