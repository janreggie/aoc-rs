use crate::util::vectors;
use anyhow::{bail, Context, Result};
use sscanf::scanf;
use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

/// A position, relative to a Scanner (at origin).
#[derive(PartialEq, Eq, Debug, Hash)]
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
    /// rotate((1,2,3), XYZ0) == (1,2,3)
    /// rotate((1,2,3), ZXY2) == (-3,1,-2)
    /// ```
    fn rotate(&self, orientation: RelativeOrientation) -> Position {
        match orientation {
            RelativeOrientation::XYZ0 => Position::new(self.x, self.y, self.z),
            RelativeOrientation::YZX0 => Position::new(self.y, self.z, self.x),
            RelativeOrientation::ZXY0 => Position::new(self.z, self.x, self.y),
            RelativeOrientation::XZY1 => Position::new(-self.x, self.z, self.y),
            RelativeOrientation::YXZ1 => Position::new(-self.y, self.x, self.z),
            RelativeOrientation::ZYX1 => Position::new(-self.z, self.y, self.x),
            RelativeOrientation::XZY2 => Position::new(self.x, -self.z, self.y),
            RelativeOrientation::YXZ2 => Position::new(self.y, -self.x, self.z),
            RelativeOrientation::ZYX2 => Position::new(self.z, -self.y, self.x),
            RelativeOrientation::XZY3 => Position::new(self.x, self.z, -self.y),
            RelativeOrientation::YXZ3 => Position::new(self.y, self.x, -self.z),
            RelativeOrientation::ZYX3 => Position::new(self.z, self.y, -self.x),
            RelativeOrientation::XYZ1 => Position::new(self.x, -self.y, -self.z),
            RelativeOrientation::YZX1 => Position::new(self.y, -self.z, -self.x),
            RelativeOrientation::ZXY1 => Position::new(self.z, -self.x, -self.y),
            RelativeOrientation::XYZ2 => Position::new(-self.x, self.y, -self.z),
            RelativeOrientation::YZX2 => Position::new(-self.y, self.z, -self.x),
            RelativeOrientation::ZXY2 => Position::new(-self.z, self.x, -self.y),
            RelativeOrientation::XYZ3 => Position::new(-self.x, -self.y, self.z),
            RelativeOrientation::YZX3 => Position::new(-self.y, -self.z, self.x),
            RelativeOrientation::ZXY3 => Position::new(-self.z, -self.x, self.y),
            RelativeOrientation::XZYA => Position::new(-self.x, -self.z, -self.y),
            RelativeOrientation::YXZA => Position::new(-self.y, -self.x, -self.z),
            RelativeOrientation::ZYXA => Position::new(-self.z, -self.y, -self.x),
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

    /// Subtracts two Positions and returns the difference.
    /// That is, `p.add(d) == q <=> q.sub(p) == d`
    fn sub(&self, rhs: &Position) -> (i32, i32, i32) {
        (self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// How another Scanner is oriented relative to another Scanner
#[derive(Clone, Copy, Debug, EnumIter)]
enum RelativeOrientation {
    // No sign changes
    XYZ0,
    YZX0,
    ZXY0,

    // Only the first value changes
    XZY1,
    YXZ1,
    ZYX1,

    // Only the second value changes
    XZY2,
    YXZ2,
    ZYX2,

    // Only the third value changes
    XZY3,
    YXZ3,
    ZYX3,

    // Only the first value doesn't change
    XYZ1,
    YZX1,
    ZXY1,

    // Only the second value doesn't change
    XYZ2,
    YZX2,
    ZXY2,

    // Only the third value doesn't change
    XYZ3,
    YZX3,
    ZXY3,

    // All of the values' signs change
    XZYA,
    YXZA,
    ZYXA,
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
        // We're trying all possibilities of `ref_s.rotate(ro).add(delta) == ref_o`.
        for ro in RelativeOrientation::iter() {
            for ref_s in &self.beacons {
                for ref_o in &other.beacons {
                    // Bruh
                }
            }
        }

        None
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let groups = vectors::group(lines);
    let scanners: Result<Vec<Scanner>> = groups.into_iter().map(|g| Scanner::new(g)).collect();
    let scanners = scanners.context("could not create scanners")?;

    // Let's test for the first two...
    let (s_0, s_1) = (&scanners[0], &scanners[1]);
    let resp = s_1.conform(s_0).context("uh oh")?;

    // Everything is relative to the zeroth Scanner...
    let standard = &scanners[0];
    let mut hashset = HashSet::new();

    for to_comp in &scanners {
        let (ro, dist) = to_comp.conform(standard).context(format!(
            "could not compare `{}` with the standard",
            to_comp.name
        ))?;
        for beacon in &to_comp.beacons {
            hashset.insert(beacon.rotate(ro).add(&dist));
        }
    }

    let ans1 = hashset.len();
    Ok((ans1.to_string(), String::from("unimplemented")))
}
