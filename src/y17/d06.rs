use std::collections::HashMap;

use crate::util::vectors;
use anyhow::{bail, Context, Result};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct MemoryBlocks(Vec<u32>);

impl MemoryBlocks {
    fn new(s: &String) -> Result<MemoryBlocks> {
        let s: Vec<u32> =
            vectors::from_strs(&s.split_ascii_whitespace().map(|s| s.to_string()).collect())
                .context("could not parse input to MemoryBlocks")?;
        Ok(MemoryBlocks(s))
    }

    fn redistribute(&mut self) {
        let len = self.0.len();
        // Get the index with the largest memory
        let mut ind = 0;
        let mut record = self.0[0];
        for ii in 0..len {
            if self.0[ii] > record {
                ind = ii;
                record = self.0[ii];
            }
        }

        // Get whatever is in ind and do some divmodding
        let (dd, mm) = (record / len as u32, record % len as u32);
        self.0[ind] = 0;
        for ii in 0..len {
            self.0[ii] += dd;
        }
        for ii in 0..mm {
            let ii = (ii as usize + 1 + ind) % len as usize;
            self.0[ii] += 1;
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expects 1 line only, got {}", lines.len())
    }
    let mut memory_blocks = MemoryBlocks::new(&lines[0]).context("erroneous input")?;
    let mut block_history = HashMap::new();

    // Part 1: Count the number of cycles before we start breaking
    let mut cycles = 0;
    while !block_history.contains_key(&memory_blocks) {
        block_history.insert(memory_blocks.clone(), cycles);
        memory_blocks.redistribute();
        cycles += 1;
    }
    let ans1 = cycles.to_string();
    let ans2 = (cycles - block_history[&memory_blocks]).to_string();

    Ok((ans1, ans2))
}
