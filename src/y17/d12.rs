use std::collections::HashSet;

use anyhow::{bail, Context, Ok, Result};
use sscanf::scanf;

struct Village {
    connections: Vec<HashSet<usize>>,
}

impl Village {
    fn new(lines: Vec<String>) -> Result<Village> {
        fn line_to_pair(line: &str) -> Result<(usize, HashSet<usize>)> {
            let (sender, recipients) =
                scanf!(line, "{} <-> {}", usize, String).context("could not parse line")?;
            let recipients = recipients
                .split(", ")
                .map(|rr| {
                    rr.parse::<usize>()
                        .with_context(|| format!("could not parse {} to string", rr))
                })
                .collect::<Result<HashSet<_>>>()
                .with_context(|| format!("could not parse line {} properly", line))?;

            Ok((sender, recipients))
        }

        let connections = lines
            .iter()
            .enumerate()
            .map(|(ii, line)| {
                let (sender, recipients) = line_to_pair(line)
                    .with_context(|| format!("could not parse line {} correctly", line))?;
                if ii != sender {
                    bail!("line {} uses sender {}", ii, sender);
                }
                Ok(recipients)
            })
            .collect::<Result<Vec<_>>>()
            .context("could not parse input correctly")?;

        Ok(Village { connections })
    }

    /// Counts how many programs are connected to program.
    fn connected_to(&self, program: usize) -> usize {
        let mut done = HashSet::new();
        let mut queue = vec![program];
        done.insert(program);
        while let Some(vv) = queue.pop() {
            let recipients = &self.connections[vv];
            for &vv in recipients {
                if !done.contains(&vv) {
                    done.insert(vv); // it'll be done anyway
                    queue.push(vv);
                }
            }
            done.insert(vv);
        }
        done.len()
    }

    /// Returns the group that some program is connected to
    fn group(&self, program: usize) -> HashSet<usize> {
        let mut done = HashSet::new();
        let mut queue = vec![program];
        done.insert(program);
        while let Some(vv) = queue.pop() {
            let recipients = &self.connections[vv];
            for &vv in recipients {
                if !done.contains(&vv) {
                    done.insert(vv); // it'll be done anyway
                    queue.push(vv);
                }
            }
            done.insert(vv);
        }
        done
    }

    fn len(&self) -> usize {
        self.connections.len()
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let village = Village::new(lines).context("could not parse to village")?;

    // Part 1: How many villages are connected to zero?
    let ans1 = village.connected_to(0).to_string();

    // Part 2: How many groups are there?
    let mut groups: Vec<HashSet<_>> = vec![];
    for program in 0..village.len() {
        let program_in_groups = groups.iter().any(|gg| gg.contains(&program));
        if !program_in_groups {
            groups.push(village.group(program));
        }
    }
    let ans2 = groups.len().to_string();

    Ok((ans1, ans2))
}
