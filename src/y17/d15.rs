use anyhow::{bail, Context, Ok, Result};

const GEN_A_FACTOR: u64 = 16807;
const GEN_B_FACTOR: u64 = 48271;
const MOD: u64 = (1 << 31) - 1;
const MASK: u64 = 0xFFFF;

fn next(n: u64, f: u64) -> u64 {
    // (n * f) % MOD

    // TODO: An explanation on why this works
    let n = n * f;
    let n = (n >> 31) + (n & MOD);
    let n = (n >> 31) + (n & MOD);
    n
}

fn next2(n: u64, f: u64, mask: u64) -> u64 {
    let mut n = next(n, f);
    while n & mask != 0 {
        n = next(n, f);
    }
    n
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 2 {
        bail!("expects 2 lines, got {}", lines.len())
    }
    let mut input = lines.into_iter();
    let start_a = input
        .next()
        .unwrap()
        .strip_prefix("Generator A starts with ")
        .and_then(|v| v.parse::<u64>().ok())
        .context("could not parse starting value for Generator A")?;
    let start_b = input
        .next()
        .unwrap()
        .strip_prefix("Generator A starts with ")
        .and_then(|v| v.parse::<u64>().ok())
        .context("could not parse starting value for Generator B")?;

    // Part 1: Number of times last 16 bits equal when multiplying
    let mut count = 0;
    let (mut gen_a, mut gen_b) = (start_a, start_b);
    for _ in 0..40_000_000 {
        gen_a = next(gen_a, GEN_A_FACTOR);
        gen_b = next(gen_b, GEN_B_FACTOR);
        if gen_a & MASK == gen_b & MASK {
            count += 1;
        }
    }
    let ans1 = Ok(count.to_string());

    // Part 2: Number of times last 16 bits will equal, while it ends in 0b11 or 0b111
    let mut count = 0;
    let (mut gen_a, mut gen_b) = (start_a, start_b);
    for _ in 0..5_000_000 {
        gen_a = next2(gen_a, GEN_A_FACTOR, 0b0011);
        gen_b = next2(gen_b, GEN_B_FACTOR, 0b0111);
        if gen_a & MASK == gen_b & MASK {
            count += 1;
        }
    }
    let ans2 = Ok(count.to_string());

    Ok((ans1, ans2))
}
