use anyhow::{bail, Context, Result};

#[derive(Debug)]
struct Spinlock {
    steps_per_input: usize,
    buffer: Vec<usize>,
    ind: usize,
}

impl Spinlock {
    fn new(steps_per_input: usize) -> Spinlock {
        Spinlock {
            steps_per_input,
            buffer: vec![0],
            ind: 0,
        }
    }

    fn next(&mut self) {
        let next_ind = (self.ind + self.steps_per_input) % self.buffer.len() + 1;
        self.buffer.insert(next_ind, self.buffer.len());
        self.ind = next_ind;
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected 1 line as input, got {} instead", lines.len())
    }
    let input = lines
        .into_iter()
        .next()
        .unwrap()
        .parse::<usize>()
        .context("could not read input as usize")?;

    let mut spinlock = Spinlock::new(input);
    for _ in 0..30 {
        spinlock.next();
        print!(
            "ind {:2} len {:2} ",
            spinlock.ind,
            spinlock.buffer.len() - 1
        );
        println!(
            "{:?}",
            spinlock
                .buffer
                .iter()
                .enumerate()
                .map(|(i, n)| if i == spinlock.ind {
                    format!("({})", *n)
                } else {
                    format!("{}", *n)
                })
                .collect::<Vec<_>>()
        );
    }
    todo!();

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

    // Part 2: After 50 million
    let mut spinlock = Spinlock::new(input);
    for _ in 0..50_0000 {
        spinlock.next();
    }
    let ans2 = spinlock.buffer[1].to_string();

    Ok((ans1, ans2))
}
