use anyhow::{bail, Context, Result};
use sscanf::scanf;

fn add_bound_10(a: u32, b: u32) -> u32 {
    (a + b - 1) % 10 + 1
}

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

/// DiracState describes the state of several universes
#[derive(Debug, Clone, Copy)]
struct DiracState {
    /// Number of universes that follow this state
    count: usize,
    /// Position of player 1
    p_1: u32,
    /// Posiiton of player 2
    p_2: u32,
}

impl DiracState {
    fn iter(&self, is_1: bool) -> [DiracState; 7] {
        // [(what to add to the positions, the number of states there'll be more of)]
        let state_multipliers: [(u32, usize); 7] =
            [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let mut result = [*self; 7];
        for ii in 0..7 {
            result[ii].count *= state_multipliers[ii].1;
            if is_1 {
                result[ii].p_1 =
                    add_bound_10(self.p_1, state_multipliers[ii].0);
            } else {
                result[ii].p_2 =
                    add_bound_10(self.p_2, state_multipliers[ii].0);
            }
        }

        result
    }
}

struct DiracMultiverse {
    thres: usize, // What should the score of the player to win
    won_1: usize, // Number of universes won by player 1
    won_2: usize, // Number of universes won by player 2
    is_1: bool,   // Checks if it is player 1's turn

    /// The multiverse proper, such that
    /// multiverse[s_2][s_1] determines the states
    /// where player 1 has score s_1 and player 2 has score s_2.
    multiverse: Vec<Vec<Vec<DiracState>>>,
}

impl DiracMultiverse {
    /// Constructor, such that Player 1, 2 start from p_1, p_2,
    /// and a player must score at least `thres` to win.
    fn new(p_1: u32, p_2: u32, thres: u32) -> DiracMultiverse {
        let thres = thres as usize;
        let mut multiverse = vec![vec![vec![]; thres]; thres];
        if thres > 0 {
            multiverse[0][0].push(DiracState { count: 1, p_1, p_2 })
        }

        DiracMultiverse { thres, won_1: 0, won_2: 0, is_1: true, multiverse }
    }

    /// Solves the problem
    fn solve(&mut self) {
        let mut count = 0;
        while !self.is_done() {
            self.iter();
            count += 1;
        }
        dbg!(count);
    }

    /// Iterates through the entire grid
    fn iter(&mut self) {
        let mut next_multiverse = vec![vec![vec![]; self.thres]; self.thres];
        for score_1 in 0..self.thres {
            for score_2 in 0..self.thres {
                let old_states = &self.multiverse[score_2][score_1];
                if old_states.len() == 0 {
                    continue;
                }
                for old_state in old_states {
                    let next_states = old_state.iter(self.is_1);
                    for next_state in next_states {
                        let (mut score_1, mut score_2) = (score_1, score_2);
                        if self.is_1 {
                            score_1 += next_state.p_1 as usize;
                            if score_1 >= self.thres {
                                self.won_1 += next_state.count;
                            } else {
                                next_multiverse[score_2][score_1]
                                    .push(next_state);
                            }
                        } else {
                            score_2 += next_state.p_2 as usize;
                            if score_2 >= self.thres {
                                self.won_2 += next_state.count;
                            } else {
                                next_multiverse[score_2][score_1]
                                    .push(next_state);
                            }
                        }
                    }
                }
            }
        }

        self.is_1 = !self.is_1;
        self.multiverse = next_multiverse;
    }

    /// Determines if all possibilities have been determined already
    fn is_done(&self) -> bool {
        for row in &self.multiverse {
            for states in row {
                if states.len() != 0 {
                    return false;
                }
            }
        }
        true
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
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
        let die_output =
            loaded_die.roll() + loaded_die.roll() + loaded_die.roll();
        if is_player_1 {
            pos_1 = add_bound_10(pos_1, die_output);
            score_1 += pos_1;
        } else {
            pos_2 = add_bound_10(pos_2, die_output);
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
    let ans1 = Ok(ans1.to_string());

    // Now for Part 2... bruh
    let mut dirac_multiverse = DiracMultiverse::new(input_1, input_2, 21);
    dirac_multiverse.solve();
    let ans2 = dirac_multiverse.won_1.max(dirac_multiverse.won_2);
    let ans2 = Ok(ans2.to_string());

    Ok((ans1, ans2))
}
