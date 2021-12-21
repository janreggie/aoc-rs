use crate::util::vectors;
use anyhow::{bail, Context, Result};
use sscanf::scanf;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A position, relative to a Scanner (at origin).
#[derive(PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }

    /// "Rotates" a position. For example,
    ///
    /// ```none
    /// rotate((1,2,3), XYZ) == (1,2,3)
    /// rotate((1,2,3), ZXY) == (3,1,2)
    /// rotate((1,2,3), XZY) == (1,3,-2) (because XZY is a "negative orientation")
    /// ```
    fn rotate(&self, orientation: &RelativeOrientation) -> Position {
        match orientation {
            RelativeOrientation::XYZ => Position::new(self.x, self.y, self.z),
            RelativeOrientation::YZX => Position::new(self.y, self.z, self.x),
            RelativeOrientation::ZXY => Position::new(self.z, self.x, self.y),
            RelativeOrientation::XZY => Position::new(self.x, self.z, -self.y),
            RelativeOrientation::YXZ => Position::new(self.y, self.x, -self.z),
            RelativeOrientation::ZYX => Position::new(self.z, self.y, -self.x),
        }
    }

    /// Adds a Position and a "difference"
    fn add(&self, delta: &(i32, i32, i32)) -> Position {
        Position {
            x: self.x + delta.0,
            y: self.y + delta.1,
            z: self.z + delta.2,
        }
    }

    /// Subtracts two Positions and returns the difference
    fn sub(&self, rhs: &Position) -> (i32, i32, i32) {
        (self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// How another Scanner is oriented relative to another Scanner
#[derive(EnumIter)]
enum RelativeOrientation {
    // Positive orientations. Signs don't change.
    XYZ,
    YZX,
    ZXY,

    // Negative orientations. The inverted val is the last param.
    XZY,
    YXZ,
    ZYX,
}

/// Scanner probed data
struct Scanner {
    name: String,
    beacons: Vec<Position>,
}

impl Scanner {
    fn new(mut lines: Vec<String>) -> Result<Scanner> {
        if lines.len() < 13 {
            bail!(
                "expects there to be at least 13 lines, got {} instead",
                lines.len()
            )
        }

        let name = lines.swap_remove(0);
        let mut beacons = Vec::new();
        for line in lines {
            let (x, y, z) = scanf!(line, "{},{},{}", i32, i32, i32)
                .context(format!("could not parse line `{}`", line))?;
            beacons.push(Position::new(x, y, z));
        }

        Ok(Scanner { name, beacons })
    }

    /// There are twelve items in `self.beacons` (each being `b_s`) that coincide with `other.beacons` (`b_o`)
    /// such that `self.conform(other) == Some((ro, (dx,dy,dz))) <=> b_s.rotate(ro).add((dx,dy,dz)) == b_o`.
    fn conform(&self, other: &Scanner) -> Option<(RelativeOrientation, (i32, i32, i32))> {
        // Here, we're trying to look for a "hook".
        for ro in RelativeOrientation::iter() {
            for ref_s in &self.beacons {
                for ref_o in &other.beacons {
                    // Okay, what should we do here..?
                }
            }
            // Do things here with ro
        }

        None
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let groups = vectors::group(lines);
    let scanners: Result<Vec<Scanner>> = groups.into_iter().map(|g| Scanner::new(g)).collect();
    let scanners = scanners.context("could not create scanners")?;

    // Now, iterate all the scanners... There are 25 of them...
    unimplemented!()
}
