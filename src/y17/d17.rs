use anyhow::{bail, Context, Result};

#[derive(Debug)]
struct Spinlock {
    steps_per_input: usize,
    buffer: Vec<usize>,
    ind: usize,
}

impl Spinlock {
    fn new(steps_per_input: usize) -> Spinlock {
        Spinlock { steps_per_input, buffer: vec![0], ind: 0 }
    }

    fn next(&mut self) {
        let next_ind =
            (self.ind + self.steps_per_input) % self.buffer.len() + 1;
        self.buffer.insert(next_ind, self.buffer.len());
        self.ind = next_ind;
    }
}

#[derive(Debug)]
struct SpinlockAtZero {
    steps_per_input: usize,
    next_to_zero: usize,
    length: usize,
    ind: usize,
}

impl SpinlockAtZero {
    fn new(steps_per_input: usize) -> Self {
        SpinlockAtZero { steps_per_input, next_to_zero: 1, length: 2, ind: 1 }
    }

    fn next(&mut self) {
        let next_ind = (self.ind + self.steps_per_input) % self.length + 1;
        if next_ind == 1 {
            self.next_to_zero = self.length; // what should be added
        }
        self.length += 1;
        self.ind = next_ind;
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expected 1 line as input, got {} instead", lines.len())
    }
    let input = lines
        .into_iter()
        .next()
        .unwrap()
        .parse::<usize>()
        .context("could not read input as usize")?;

    // Part 1: After 2017 iterations
    let mut spinlock = Spinlock::new(input);
    for _ in 0..2017 {
        spinlock.next();
    }
    let ans1 = spinlock
        .buffer
        .get(spinlock.ind + 1)
        .cloned()
        .unwrap_or(spinlock.buffer[0])
        .to_string();
    let ans1 = Ok(ans1);

    // Part 2: After 50 million.
    let mut spinlock = SpinlockAtZero::new(input);
    while spinlock.length <= 50_000_000 {
        spinlock.next();
    }
    let ans2 = Ok(spinlock.next_to_zero.to_string());

    Ok((ans1, ans2))
}
