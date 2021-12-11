use anyhow::{bail, Context, Result};
use std::fmt;

struct Octopuses {
    energy_levels: [[u8; 10]; 10],
}

impl Octopuses {
    fn new(lines: Vec<String>) -> Result<Octopuses> {
        if lines.len() != 10 {
            bail!(
                "expected lines to be of length 10, got {} instead",
                lines.len()
            )
        }

        let mut energy_levels = [[0; 10]; 10];
        for ii in 0..10 {
            let line = lines.get(ii).unwrap();
            if line.len() != 10 {
                bail!(
                    "expected line `{}` to be of length 10, got {} instead",
                    line,
                    line.len()
                )
            }
            let line: Vec<char> = line.chars().collect();
            for jj in 0..10 {
                let ch = line[jj];
                match ch.to_digit(10) {
                    None => bail!("could not parse character `{}` from line `{:?}`", ch, line),
                    Some(d) => energy_levels[ii][jj] = d as u8,
                }
            }
        }

        Ok(Octopuses { energy_levels })
    }

    /// Returns how many octopuses have flashed
    fn next(&mut self) -> u32 {
        let mut flash_queue = Vec::new();
        for y in 0..10 {
            for x in 0..10 {
                self.energy_levels[y][x] += 1;
                if self.energy_levels[y][x] == 10 {
                    flash_queue.push((x, y));
                }
            }
        }

        let mut count_flashed = 0;
        let mut flashed = [[false; 10]; 10];
        while let Some((x, y)) = flash_queue.pop() {
            flashed[y][x] = true;
            count_flashed += 1;

            // Increment the neighbors...
            // North
            if y > 0 && !flashed[y - 1][x] {
                self.energy_levels[y - 1][x] += 1;
                if self.energy_levels[y - 1][x] == 10 {
                    flash_queue.push((x, y - 1));
                }
            }
            // South
            if y < 9 && !flashed[y + 1][x] {
                self.energy_levels[y + 1][x] += 1;
                if self.energy_levels[y + 1][x] == 10 {
                    flash_queue.push((x, y + 1));
                }
            }
            // West
            if x > 0 && !flashed[y][x - 1] {
                self.energy_levels[y][x - 1] += 1;
                if self.energy_levels[y][x - 1] == 10 {
                    flash_queue.push((x - 1, y));
                }
            }
            // East
            if x < 9 && !flashed[y][x + 1] {
                self.energy_levels[y][x + 1] += 1;
                if self.energy_levels[y][x + 1] == 10 {
                    flash_queue.push((x + 1, y));
                }
            }
            // North-West
            if x > 0 && y > 0 && !flashed[y - 1][x - 1] {
                self.energy_levels[y - 1][x - 1] += 1;
                if self.energy_levels[y - 1][x - 1] == 10 {
                    flash_queue.push((x - 1, y - 1));
                }
            }
            // North-East
            if x < 9 && y > 0 && !flashed[y - 1][x + 1] {
                self.energy_levels[y - 1][x + 1] += 1;
                if self.energy_levels[y - 1][x + 1] == 10 {
                    flash_queue.push((x + 1, y - 1));
                }
            }
            // South-West
            if x > 0 && y < 9 && !flashed[y + 1][x - 1] {
                self.energy_levels[y + 1][x - 1] += 1;
                if self.energy_levels[y + 1][x - 1] == 10 {
                    flash_queue.push((x - 1, y + 1));
                }
            }
            // South-East
            if x < 9 && y < 9 && !flashed[y + 1][x + 1] {
                self.energy_levels[y + 1][x + 1] += 1;
                if self.energy_levels[y + 1][x + 1] == 10 {
                    flash_queue.push((x + 1, y + 1));
                }
            }
        }

        // Finally, set those flashed to zero
        for y in 0..10 {
            for x in 0..10 {
                if flashed[y][x] {
                    self.energy_levels[y][x] = 0;
                }
            }
        }

        count_flashed
    }
}

impl fmt::Display for Octopuses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..10 {
            for x in 0..10 {
                write!(f, "{}", self.energy_levels[y][x])?
            }
            writeln!(f, "")?
        }

        write!(f, "")
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut octopuses = Octopuses::new(lines).context("could not create octopuses")?;

    // Part 1: How many total flashes after 100 steps.
    // Part 2: When the result becomes 100, since that's when *all* flash.
    let mut ans1 = 0;
    let mut ans2 = 0;
    for ii in 0..100 {
        let count_flashed = octopuses.next();
        ans1 += count_flashed;
        if count_flashed == 100 && ans2 == 0 {
            ans2 = ii + 1;
        }
    }

    if ans2 == 0 {
        for ii in 100.. {
            let count_flashed = octopuses.next();
            if count_flashed == 100 && ans2 == 0 {
                ans2 = ii + 1;
                break;
            }
        }
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
