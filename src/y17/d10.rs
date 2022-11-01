use anyhow::{bail, Context, Ok, Result};

use crate::util::vectors;

#[derive(Debug)]
struct Knot {
    numbers: [u8; 256],
    position: usize,
    skip_size: usize,
}

impl Knot {
    fn new() -> Knot {
        let mut numbers = [0; 256];
        for ii in 0..256 {
            numbers[ii] = ii as u8;
        }
        Knot {
            numbers,
            position: 0,
            skip_size: 0,
        }
    }

    /// Twist a span of numbers from its internal position with length provided.
    fn twist(&mut self, length: usize) {
        // Swap the numbers across the length
        for ii in 0..length / 2 {
            let (lhs, rhs) = (self.position + ii, self.position + length - 1 - ii);
            let (lhs, rhs) = (lhs % 256, rhs % 256);
            (self.numbers[lhs], self.numbers[rhs]) = (self.numbers[rhs], self.numbers[lhs]);
        }

        // Increment position and skip size
        self.position = (self.position + length + self.skip_size) % 256;
        self.skip_size = (self.skip_size + 1) % 256;
    }

    fn dense_hash(&self) -> [u8; 16] {
        let mut result = [0; 16];
        for ii in 0..256 {
            result[ii / 16] ^= self.numbers[ii];
        }
        result
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected only 1 line as input, got {}", lines.len())
    }
    let input = lines.into_iter().next().unwrap();

    // Part 1: Do the sequence as usual; multiply the first 2 values
    let mut knot = Knot::new();
    let lengths = vectors::split_and_trim(&input, ',');
    let lengths: Vec<usize> = vectors::from_strs(&lengths)
        .with_context(|| format!("could not format {:?} as input", &input))?;
    for length in &lengths {
        knot.twist(*length);
    }
    let ans1 = (knot.numbers[0] as u32 * knot.numbers[1] as u32).to_string();

    // Part 2: Interpret input as ascii, with additional lengths, 64 times, etc.
    let mut lengths = input
        .as_bytes()
        .iter()
        .map(|x| *x as usize)
        .collect::<Vec<_>>();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut knot = Knot::new();
    for _ in 0..64 {
        for length in &lengths {
            knot.twist(*length);
        }
    }
    let dense_hash = knot.dense_hash();
    let ans2 = dense_hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join("");

    Ok((ans1, ans2))
}
