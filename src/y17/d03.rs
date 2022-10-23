use std::collections::HashMap;

use anyhow::{bail, Result};

// Part 1: Get the location of a number in the infinite grid
fn number_to_location(input: u32) -> (i32, i32) {
    // (2a+1)^2 is the smallest odd square that isn't greater than input
    let input = input as i32;
    let a = ((f64::from(input).sqrt() - 1.0) / 2.0).floor() as i32;
    let (mut x, mut y) = (a, -a);
    // ptr is our point in the grid
    let mut ptr = (2 * a + 1) * (2 * a + 1);

    // Zeroth move: go to the next square
    if ptr < input {
        ptr += 1;
        x += 1;
    }
    // First move: go upwards
    if ptr < input {
        let to_add = (2 * a + 1).min(input - ptr);
        ptr += to_add;
        y += to_add;
    }
    // Second move: go leftwards
    if ptr < input {
        let to_add = (2 * a + 2).min(input - ptr);
        ptr += to_add;
        x -= to_add;
    }
    // Third move: go downwards
    if ptr < input {
        let to_add = (2 * a + 2).min(input - ptr);
        ptr += to_add;
        y -= to_add;
    }
    // Fourth move: go rightwards
    if ptr < input {
        let to_add = (2 * a + 2).min(input - ptr);
        x += to_add;
    }

    (x, y)
}

// Part 2: Create an Iterator that goes around the grid
struct SpiralMemory {
    grid: HashMap<(i32, i32), u32>,
    pos: (i32, i32),
}

impl SpiralMemory {
    pub fn new() -> SpiralMemory {
        SpiralMemory {
            grid: HashMap::new(),
            pos: (0, 0),
        }
    }

    fn next_pos((x, y): (i32, i32)) -> (i32, i32) {
        // Case 0: pos moves to the next square
        if x >= 0 && x == -y {
            return (x + 1, y);
        }
        // Case 1: pos could go upwards
        if x > y && x + y >= 0 {
            return (x, y + 1);
        }
        // Case 2: pos can go leftwards
        if x <= y && x + y > 0 {
            return (x - 1, y);
        }
        // Case 3: pos can go downwards
        if x < y && x + y <= 0 {
            return (x, y - 1);
        }
        // Case 4: pos can go leftwards
        if x >= y && x + y <= 0 {
            return (x + 1, y);
        }

        unimplemented!("unexpected case with x=={x} and y=={y}");
    }
}

impl Iterator for SpiralMemory {
    type Item = ((i32, i32), u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == (0, 0) {
            self.grid.insert(self.pos, 1);
            self.pos = (1, 0);
            return Some((self.pos, 1));
        }

        let neighbors = [
            (self.pos.0 - 1, self.pos.1 - 1),
            (self.pos.0 - 1, self.pos.1),
            (self.pos.0 - 1, self.pos.1 + 1),
            (self.pos.0, self.pos.1 - 1),
            (self.pos.0, self.pos.1 + 1),
            (self.pos.0 + 1, self.pos.1 - 1),
            (self.pos.0 + 1, self.pos.1),
            (self.pos.0 + 1, self.pos.1 + 1),
        ];
        let neighbor_sum: u32 = neighbors
            .into_iter()
            .map(|p| self.grid.get(&p).unwrap_or(&0))
            .sum();
        self.grid.insert(self.pos, neighbor_sum);
        self.pos = SpiralMemory::next_pos(self.pos);

        Some((self.pos, neighbor_sum))
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("input should be only one line")
    }
    let input: u32 = lines[0].parse().unwrap();

    // Part 1: Get the Manhattan distance
    let (x, y) = number_to_location(input);
    let ans1 = (x.abs() + y.abs()).to_string();

    // Part 2: Iterate throughout the spiral memory
    let ans2;
    let mut spiral_memory = SpiralMemory::new();
    loop {
        let (_pos, value) = spiral_memory.next().unwrap();
        if value > input {
            ans2 = value.to_string();
            break;
        }
    }
    Ok((ans1, ans2))
}
