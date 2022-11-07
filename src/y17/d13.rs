use anyhow::{bail, Context, Ok, Result};
use num::Integer;
use sscanf::scanf;

struct Firewall {
    /// Vec<(depth, range)>
    layers: Vec<(u32, u32)>,
}

impl Firewall {
    fn new() -> Firewall {
        Firewall { layers: vec![] }
    }

    fn add(&mut self, depth: u32, range: u32) {
        self.layers.push((depth, range))
    }

    /// Evaluates the severity when "hitching a ride" on a packet after some delay in picoseconds.
    fn severity(&self, delay: u32) -> u32 {
        let mut ans = 0;
        for &(depth, range) in &self.layers {
            if range == 0 {
                continue;
            }
            let time = depth + delay;
            let is_going_up = (time / (range - 1)) % 2 == 0; // whether the scanner is going up
            let scanner_position: u32 = if is_going_up {
                time % (range - 1)
            } else {
                (range - 1) - (time % (range - 1))
            };
            if scanner_position == 0 {
                ans += time * range;
            }
        }

        ans
    }

    /// Evaluates the ideal delay so that an intruder would not trip the firewall up.
    /// Warning: Takes time proportional to the LCM of the layers' ranges.
    /// TODO: Improve using modular arithmetic.
    fn ideal_delay(&self) -> Option<u32> {
        let lcm = self
            .layers
            .iter()
            .map(|(_d, r)| (r - 1) * 2)
            .fold(1, |acc, x| acc.lcm(&x));
        for delay in 0..lcm {
            let mut has_bypassed = true;
            for &(depth, range) in &self.layers {
                if (delay + depth) % (2 * (range - 1)) == 0 {
                    has_bypassed = false;
                    break;
                }
            }
            if has_bypassed {
                return Some(delay);
            }
        }

        None
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut firewall = Firewall::new();
    let layers = lines
        .iter()
        .map(|line| {
            scanf!(line, "{}: {}", u32, u32)
                .with_context(|| format!("could not parse line '{}'", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not parse input")?;
    for (depth, range) in layers {
        firewall.add(depth, range);
    }

    // Part 1: Severity at zero delays
    let ans1 = firewall.severity(0).to_string();

    // Part 2: Lowest delay without getting caught
    let ans2 = firewall
        .ideal_delay()
        .context("firewall cannot be bypassed")?
        .to_string();

    Ok((ans1, ans2))
}
