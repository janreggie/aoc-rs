use anyhow::{bail, Context, Result};
use sscanf::scanf;

use crate::util::vectors::group;

/// Instruction is of form "move {self.count} from {self.from} to {self.to}".
/// Note that "from" and "to" are 1-indexed.
#[derive(Debug)]
struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn new(input: &str) -> Result<Instruction> {
        scanf!(input, "move {} from {} to {}", usize, usize, usize)
            .map(|(count, from, to)| Instruction { count, from, to })
            .context("could not parse input")
    }
}

type Crate = char;

/// Stacks represents the setup for crates
struct Stacks {
    // each "column" in crates are implemented with the "bottommost" crate being the zeroth element i.e., `crates[ii][0]`.
    crates: Vec<Vec<Crate>>,
}

impl Stacks {
    /// new is a not-so-accurate of generating Stacks from input
    fn new(input: &Vec<String>) -> Result<Stacks> {
        if input.len() < 2 {
            bail!(
                "expect input to be at least length 2, got {} instead",
                input.len()
            );
        }

        // Check the final string to see how many crates are there...
        let column_count = input.last().unwrap().split_ascii_whitespace().count();
        let mut crates = vec![Vec::new(); column_count];

        // Now populate the crates
        for row in 0..input.len() - 1 {
            let row = &input[row];
            for col in 0..column_count {
                let current = &row[col * 4..col * 4 + 3];
                if current == "   " {
                    continue;
                }
                let crate_val = scanf!(current, "[{}]", Crate)
                    .with_context(|| format!("unable to parse {}", current))?;
                crates[col].push(crate_val);
            }
        }
        // Finally, reverse all crates
        let crates = crates
            .into_iter()
            .map(|col| col.into_iter().rev().collect())
            .collect();

        Ok(Stacks { crates })
    }

    /// Performs instruction by moving one at a time
    fn perform(&mut self, instruction: &Instruction) -> Result<()> {
        self.validate_instr(instruction)?;
        // Now, perform
        for _ in 0..instruction.count {
            let crate_val = self.crates[instruction.from - 1].pop().unwrap();
            self.crates[instruction.to - 1].push(crate_val);
        }

        Ok(())
    }

    /// Performs instruction by moving multiple crates at once
    fn perform_quickly(&mut self, instruction: &Instruction) -> Result<()> {
        self.validate_instr(instruction)?;
        // Now perform
        let (from_ind, to_ind) = (instruction.from - 1, instruction.to - 1);
        let from_len = self.crates[from_ind].len();
        let mut moved_crates = self.crates[from_ind]
            .drain(from_len - instruction.count..)
            .collect::<Vec<_>>();
        self.crates[to_ind].append(&mut moved_crates);

        Ok(())
    }

    fn validate_instr(&self, instruction: &Instruction) -> Result<()> {
        if instruction.from > self.crates.len() || instruction.to > self.crates.len() {
            bail!("invalid instruction {:?}", instruction);
        }
        let from_len = self.crates[instruction.from - 1].len();
        if from_len < instruction.count {
            bail!(
                "too few crates in column {}; wanted {}, got {}",
                instruction.from,
                instruction.count,
                from_len
            )
        }

        Ok(())
    }

    /// Returns a single string representing the topmost crate for each column
    fn top_of_each(&self) -> String {
        self.crates
            .iter()
            .map(|col| col.last().cloned().unwrap_or(' '))
            .collect::<String>()
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let input = group(lines);
    if input.len() != 2 {
        bail!(
            "expects input to be divided in 2 groups, got {} instead",
            input.len()
        )
    }
    let mut input = input.into_iter();
    let initial_stacks = input.next().unwrap();
    let instructions = input
        .next()
        .unwrap()
        .iter()
        .map(|instr| {
            Instruction::new(instr)
                .with_context(|| format!("could not parse {} as instruction", instr))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not generate instructions")?;

    // Part 1
    let mut stacks = Stacks::new(&initial_stacks).context("could not generate stacks")?;
    for instruction in &instructions {
        stacks
            .perform(instruction)
            .with_context(|| format!("could not perform instruction {:?}", instruction))?;
    }
    let ans1 = stacks.top_of_each();

    // Part 2
    let mut stacks = Stacks::new(&initial_stacks).context("could not generate stacks")?;
    for instruction in &instructions {
        stacks
            .perform_quickly(instruction)
            .with_context(|| format!("could not perform instruction {:?}", instruction))?;
    }
    let ans2 = stacks.top_of_each();

    Ok((ans1, ans2))
}
