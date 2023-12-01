use anyhow::{bail, Context, Ok, Result};

/// Hexagonal grid which uses an axial coordinate system. See <https://math.stackexchange.com/a/2643016>.
#[derive(Debug)]
struct HexPosition {
    n: i32,  // How far north is the position
    ne: i32, // How far north-east is the position
}

impl HexPosition {
    fn new() -> HexPosition {
        HexPosition { n: 0, ne: 0 }
    }

    fn move_to(&mut self, direction: &Direction) {
        match direction {
            Direction::NW => {
                self.n += 1;
                self.ne -= 1
            }
            Direction::N => self.n += 1,
            Direction::NE => self.ne += 1,
            Direction::SW => self.ne -= 1,
            Direction::S => self.n -= 1,
            Direction::SE => {
                self.n -= 1;
                self.ne += 1
            }
        }
    }

    /// Gets the manhattan distance from the origin
    fn manhattan(&self) -> i32 {
        if (self.n <= 0) ^ (self.ne >= 0) {
            (self.n + self.ne).abs()
        } else {
            self.n.abs().max(self.ne.abs())
        }
    }
}

enum Direction {
    NW,
    N,
    NE,
    SW,
    S,
    SE,
}

impl Direction {
    /// Parses input "n", "nw", ... into a Direction
    fn new(input: &str) -> Result<Direction> {
        match input {
            "nw" => Ok(Direction::NW),
            "n" => Ok(Direction::N),
            "ne" => Ok(Direction::NE),
            "sw" => Ok(Direction::SW),
            "s" => Ok(Direction::S),
            "se" => Ok(Direction::SE),
            _ => bail!("invalid input {}", input),
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expected only 1 line as input, got {}", lines.len())
    }
    let input = lines.into_iter().next().unwrap();
    let directions = input
        .split(',')
        .map(|dd| Direction::new(dd))
        .collect::<Result<Vec<_>>>()
        .context("could not parse input")?;
    let mut hex_pos = HexPosition::new();

    // Part 1: Fewest number of steps to reach the child process by the end.
    // Part 2: Furthest child has ever got.
    let mut record_distance = i32::MIN;
    for direction in &directions {
        hex_pos.move_to(direction);
        let distance = hex_pos.manhattan();
        if distance > record_distance {
            record_distance = distance;
        }
    }
    let ans1 = Ok(hex_pos.manhattan().to_string());
    let ans2 = Ok(record_distance.to_string());

    Ok((ans1, ans2))
}
