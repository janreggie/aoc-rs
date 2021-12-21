use crate::util::vectors;
use anyhow::{bail, Context, Result};
use sscanf::scanf;

struct Loaded {
    val: u32,
    counter: usize,
}

impl Loaded {
    fn new() -> Loaded {
        Loaded { val: 1, counter: 0 }
    }

    fn roll(&mut self) -> u32 {
        let out = self.val;
        self.val += 1;
        if self.val > 100 {
            self.val = 1;
        }
        self.counter += 1;

        out
    }

    fn count(&self) -> usize {
        self.counter
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    // Parse input
    if lines.len() != 2 {
        bail!("expected input to be two lines, got {}", lines.len())
    }
    let input_1 = scanf!(&lines[0], "Player 1 starting position: {}", u32)
        .context("could not parse player 1 starting pos")?;
    let input_2 = scanf!(&lines[1], "Player 2 starting position: {}", u32)
        .context("could not parse player 2 starting pos")?;

    // Part 1 is relatively straightforward
    let (mut score_1, mut score_2) = (0, 0);
    let (mut pos_1, mut pos_2) = (input_1, input_2);
    let mut loaded_die = Loaded::new();
    let mut is_player_1 = true;
    while score_1 < 1000 && score_2 < 1000 {
        if is_player_1 {
            pos_1 += loaded_die.roll() + loaded_die.roll() + loaded_die.roll();
            pos_1 = (pos_1 - 1) % 10 + 1;
            score_1 += pos_1;
        } else {
            pos_2 += loaded_die.roll() + loaded_die.roll() + loaded_die.roll();
            pos_2 = (pos_2 - 1) % 10 + 1;
            score_2 += pos_2;
        }
        is_player_1 = !is_player_1;
    }
    let ans1;
    if score_1 >= 1000 {
        ans1 = score_2 * loaded_die.count() as u32
    } else {
        ans1 = score_1 * loaded_die.count() as u32
    }

    // Now for Part 2... bruh

    Ok((ans1.to_string(), String::from("undefined")))
}
