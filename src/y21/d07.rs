use crate::util::vectors;
use anyhow::{bail, Context, Result};

struct CrabPositions {
    positions: Vec<u32>,
    counts: Vec<u32>, // counts[x] == number of crabs at pos. positions[x]
}

impl CrabPositions {
    fn new(mut poss: Vec<u32>) -> Result<CrabPositions> {
        if poss.len() == 0 {
            bail!("empty positions")
        }
        poss.sort_unstable();

        let mut positions = Vec::new();
        let mut counts = Vec::new();

        for pos in poss {
            if positions.len() == 0 {
                positions.push(pos);
                counts.push(1);
                continue;
            }
            let ind = positions.len() - 1;
            if positions[ind] == pos {
                counts[ind] += 1;
            } else {
                positions.push(pos);
                counts.push(1);
            }
        }

        Ok(CrabPositions { positions, counts })
    }

    /// Computes the fuel that the crabs will have to spend
    /// if the amount of fuel that a crab has to spend
    /// is linearly proportional to its distance to said position.
    fn compute_fuel_lin(&self, pos: u32) -> u32 {
        let fuel = |p: u32| if p > pos { p - pos } else { pos - p };

        let mut result = 0;
        for ii in 0..self.positions.len() {
            result += fuel(self.positions[ii]) * self.counts[ii];
        }
        result
    }

    /// Computes the fuel that the crabs will have to spend
    /// if the amount of fuel that a crab has to spend
    /// is proportional to the **square** of its distance to said position.
    fn compute_fuel_sq(&self, pos: u32) -> u32 {
        let ss = |x: u32| x * (x + 1) / 2; // fuel is triangular numbers
        let fuel = |p: u32| ss(if p > pos { p - pos } else { pos - p });

        let mut result = 0;
        for ii in 0..self.positions.len() {
            result += fuel(self.positions[ii]) * self.counts[ii];
        }
        result
    }

    // For the below methods,
    // you might want to read on minimizing and maximizing.
    // <http://www1.udel.edu/nag/ohucl05pd/c/Manual/E04/e04int_cl05.pdf> is a good resource.

    fn find_ideal_lin(&self) -> u32 {
        // TODO: The median minimizes the sum of absolute deviations.
        // That solution would be of linear time,
        // compared to the one below which is quadratic.

        let mut result = u32::MAX;
        for pos in &self.positions {
            let pos = *pos;
            let current = self.compute_fuel_lin(pos);
            if current < result {
                result = current
            } else {
                break;
            }
        }

        result
    }

    fn find_ideal_sq(&self) -> u32 {
        // TODO: The mean minimizes the mean squared error.
        // See <https://math.stackexchange.com/a/967182> on what I mean by that.
        // That solution would be of linear time,
        // compared to the one below which is quadratic.

        let mut result = u32::MAX;
        let lowest_pos = *self.positions.first().unwrap();
        let highest_pos = *self.positions.last().unwrap();
        for pos in lowest_pos..highest_pos + 1 {
            let current = self.compute_fuel_sq(pos);
            if current < result {
                result = current
            } else {
                break;
            }
        }

        result
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expected lines to be of length 1, got {}", lines.len());
    }
    let crab_positions = vectors::split_and_trim(&lines[0], ',');
    let crab_positions = vectors::from_strs(&crab_positions)
        .context("could not parse crab positions as vector of u32")?;
    let crab_positions = CrabPositions::new(crab_positions)?;

    let ans1 = Ok(crab_positions.find_ideal_lin().to_string());
    let ans2 = Ok(crab_positions.find_ideal_sq().to_string());
    Ok((ans1, ans2))
}
