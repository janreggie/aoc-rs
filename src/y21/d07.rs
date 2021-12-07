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
        let ss = |x: u32| x * (x + 1) / 2;
        let fuel = |p: u32| ss(if p > pos { p - pos } else { pos - p });

        let mut result = 0;
        for ii in 0..self.positions.len() {
            result += fuel(self.positions[ii]) * self.counts[ii];
        }
        result
    }

    // TODO: Can you find a linear time solution?
    fn find_ideal_lin(&self) -> u32 {
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

    // TODO: Can you find a linear time solution?
    fn find_ideal_sq(&self) -> u32 {
        let mut result = u32::MAX;
        for pos in &self.positions {
            let pos = *pos;
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

pub fn d07(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected lines to be of length 1, got {}", lines.len());
    }
    let crab_positions = vectors::split_and_trim(&lines[0], ',');
    let crab_positions = vectors::from_strs(&crab_positions)
        .context("could not parse crab positions as vector of u32")?;
    let crab_positions = CrabPositions::new(crab_positions)?;

    let ans1 = crab_positions.find_ideal_lin();
    let ans2 = crab_positions.find_ideal_sq();
    Ok((ans1.to_string(), ans2.to_string()))
}
