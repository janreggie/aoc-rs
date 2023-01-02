use anyhow::{bail, Context, Result};
use sscanf::scanf;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

struct Rope<const LENGTH: usize> {
    /// knots[0] represents the head
    knots: [Pos; LENGTH],
    tailed_positions: HashSet<Pos>,
}

impl<const LENGTH: usize> Rope<LENGTH> {
    const LENGTH: usize = LENGTH;

    fn new() -> Self {
        assert!(LENGTH > 0, "length must be greater than zero");
        let zero_pos = Pos::new(0, 0);
        let mut tailed_positions = HashSet::new();
        tailed_positions.insert(zero_pos);
        Self {
            knots: [zero_pos; LENGTH],
            tailed_positions,
        }
    }

    fn perform(&mut self, instruction: &Instruction) {
        let (direction, step_count) = (instruction.direction, instruction.step_count);
        println!("Performing instruction {:?}", instruction);
        for _ in 0..step_count {
            // First, move head
            self.knots[0] = self.knots[0].move_to(direction);

            // Now, for the rest of the knots...
            for ii in 1..LENGTH {
                let (prev_knot, cur_knot) = (self.knots[ii - 1], self.knots.get_mut(ii).unwrap());
                match prev_knot.manhattan(&cur_knot) {
                    0..=1 => {} // Do nothing
                    2 => {
                        // If they're in the same row/column, move
                        if prev_knot.x == cur_knot.x || prev_knot.y == cur_knot.y {
                            *cur_knot = prev_knot.midpoint(*cur_knot);
                        }

                        // Otherwise, do nothing; they're still "close enough" to each other
                    }
                    _ => {
                        // Now this is where it's a bit tricky though...
                        let diff = prev_knot - *cur_knot;
                        if diff.x.abs() == diff.y.abs() {
                            cur_knot.x += diff.x - diff.x.signum();
                            cur_knot.y += diff.y - diff.y.signum();
                        } else {
                            // Now...
                        }
                        // if diff.x.abs() == 2 {
                        //     diff.x /= 2;
                        // } else {
                        //     diff.y /= 2;
                        // }
                        // *cur_knot = *cur_knot + diff;
                    }
                }
            }

            // Finally, add the tail knot
            self.tailed_positions.insert(*self.knots.last().unwrap());
            println!("Rope is now at {:?}", self.knots);
        }
    }
}

struct Grid {
    head_pos: Pos,
    tail_pos: Pos,
    tailed_positions: HashSet<Pos>,
}

impl Grid {
    fn new() -> Grid {
        let mut tailed_positions = HashSet::new();
        tailed_positions.insert(Pos::default()); // initially at (0,0)
        Grid {
            head_pos: Pos::default(),
            tail_pos: Pos::default(),
            tailed_positions,
        }
    }

    fn perform(&mut self, instruction: &Instruction) {
        let (direction, step_count) = (instruction.direction, instruction.step_count);
        for _ in 0..step_count {
            // First, move head
            let next_head_pos = self.head_pos.move_to(direction);
            self.head_pos = next_head_pos;

            // Now, tail should follow
            match self.head_pos.manhattan(&self.tail_pos) {
                0..=1 => {} // Do nothing
                2 => {
                    // If they're in the same row/column, move
                    if self.head_pos.x == self.tail_pos.x || self.head_pos.y == self.tail_pos.y {
                        self.tail_pos = self.head_pos.midpoint(self.tail_pos);
                    }

                    // Otherwise, do nothing; they're still "close enough" to each other
                }
                3 => {
                    // Now this is where it's a bit tricky though...
                    let mut diff = self.head_pos - self.tail_pos;
                    if diff.x.abs() == 2 {
                        diff.x /= 2;
                    } else {
                        diff.y /= 2;
                    }
                    self.tail_pos = self.tail_pos + diff;
                }
                _ => panic!(
                    "head {:?} and tail {:?} too far apart",
                    self.head_pos, self.tail_pos
                ),
            }
            self.tailed_positions.insert(self.tail_pos);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn move_to(&self, direction: Direction) -> Self {
        match direction {
            Direction::Left => Pos::new(self.x - 1, self.y),
            Direction::Up => Pos::new(self.x, self.y + 1),
            Direction::Right => Pos::new(self.x + 1, self.y),
            Direction::Down => Pos::new(self.x, self.y - 1),
        }
    }

    fn manhattan(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn midpoint(self, other: Self) -> Self {
        let new = self + other;
        Pos {
            x: new.x / 2,
            y: new.y / 2,
        }
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    step_count: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Instruction {
    fn new(input: &str) -> Result<Instruction> {
        let (direction, step_count) =
            scanf!(input, "{} {}", String, i32).context("could not parse input")?;
        let direction = match direction.as_str() {
            "L" => Direction::Left,
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            _ => bail!("invalid instruction {}", direction),
        };

        Ok(Instruction {
            direction,
            step_count,
        })
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let instructions = lines
        .iter()
        .map(|line| {
            Instruction::new(line).with_context(|| format!("could not parse line {}", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not parse input")?;

    // Part 1: Perform all instructions for rope with length 2
    let mut rope = Rope::<2>::new();
    for instruction in &instructions {
        rope.perform(instruction);
    }
    let ans1 = rope.tailed_positions.len().to_string();

    // Part 2: Perform all instructions for rope with length 10
    todo!("implement me");
    let mut rope = Rope::<10>::new();
    for instruction in &instructions {
        rope.perform(instruction);
    }
    let ans2 = rope.tailed_positions.len().to_string();

    Ok((ans1, ans2))
}
