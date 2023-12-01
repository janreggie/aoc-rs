use anyhow::{bail, Context, Result};
use sscanf::scanf;

#[derive(Debug)]
struct TargetArea {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl TargetArea {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Result<TargetArea> {
        if x1 < 0 || x2 < 0 {
            bail!("x1 and x2 must be non-negative, got {} and {}", x1, x2);
        }
        if x1 > x2 || y1 > y2 {
            bail!("invalid range");
        }

        Ok(TargetArea { x1, x2, y1, y2 })
    }

    /// Check if it's possible for a probe to move with some velocity,
    /// and if so, returns the (lowest) number of rounds that it'll take to reach the target area.
    fn probe(&self, mut vx: i32, mut vy: i32) -> Option<u32> {
        let (mut px, mut py) = (0, 0); // positions

        // Check if it is feasible using vx
        let mut rounds = 0; // number of rounds
        while px < self.x1 {
            if vx <= 0 {
                return None;
            }
            px += vx;
            vx -= 1;
            rounds += 1;
        }

        for _ in 0..rounds {
            py += vy;
            vy -= 1;
        }

        // If the probe is past y1 or x2, there's no more hope.
        while py >= self.y1 && px <= self.x2 {
            if py >= self.y1 && py <= self.y2 {
                return Some(rounds);
            }
            if vx > 0 {
                px += vx;
                vx -= 1;
            }
            py += vy;
            vy -= 1;
            rounds += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe() {
        let target_area = TargetArea::new(20, 30, -10, -5).unwrap();
        assert_eq!(target_area.probe(7, 2), Some(7));
        assert_eq!(target_area.probe(6, 3), Some(9));
        assert_eq!(target_area.probe(9, 0), Some(4));
        assert_eq!(target_area.probe(17, -4), None);
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expect there to be 1 line, got {}", lines.len())
    }
    let mut lines = lines;
    let input = lines.pop().unwrap();
    let (x1, x2, y1, y2) =
        scanf!(&input, "target area: x={}..{}, y={}..{}", i32, i32, i32, i32)
            .context("unable to parse input")?;
    let target_area = TargetArea::new(x1, x2, y1, y2)
        .context("could not create target area")?;

    // Most number of rounds can be achieved by setting some vx
    // such that Triangle(vx) is between x1 and x2.
    // Note that Triangle(x) == x*(x+1)/2.
    // Least number of rounds is just (vx,vy) = (x2,y1).
    let vx_min = (x1 as f64).sqrt() as i32 - 1;
    let vx_max = x2;
    let mut count_velocities = 0;
    let mut record_height = 0;
    for vx in vx_min..(vx_max + 1) {
        for vy in y1..(-y1) {
            if let Some(_) = target_area.probe(vx, vy) {
                record_height = record_height.max(vy * (vy + 1) / 2);
                count_velocities += 1;
            }
        }
    }
    let ans1 = Ok(record_height.to_string());
    let ans2 = Ok(count_velocities.to_string());

    Ok((ans1, ans2))
}
