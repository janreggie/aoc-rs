use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Caves {
    paths: HashMap<String, HashSet<String>>,
}

impl Caves {
    fn new(lines: Vec<String>) -> Result<Caves> {
        let mut paths = HashMap::new();

        for line in lines {
            let mut nodes = vectors::split_and_trim(&line, '-');
            if nodes.len() != 2 {
                bail!(
                    "expected {} to be split into 2, got {}",
                    line,
                    nodes.len()
                );
            }

            let n1 = nodes.pop().unwrap();
            let n2 = nodes.pop().unwrap();
            let p = paths.entry(n1.clone()).or_insert(HashSet::new());
            p.insert(n2.clone());
            let p = paths.entry(n2.clone()).or_insert(HashSet::new());
            p.insert(n1.clone());
        }

        Ok(Caves { paths })
    }

    fn part_1(&self) -> usize {
        self.part_1_iter(Vec::new(), "start").0
    }

    fn part_1_iter(
        &self,
        mut stack: Vec<String>,
        current: &str,
    ) -> (usize, Vec<String>) {
        let mut result = 0;

        for next in self.paths.get(current).unwrap() {
            if next == "end" {
                result += 1;
                continue;
            }
            if next == "start" {
                continue;
            }
            if stack.contains(next) && !Caves::is_big(next) {
                continue;
            }

            stack.push(String::from(next));
            let (partial, next_stack) = self.part_1_iter(stack, next);
            stack = next_stack;
            stack.pop();
            result += partial;
        }

        (result, stack)
    }

    fn part_2(&self) -> usize {
        self.part_2_iter(Vec::new(), "start", false).0
    }

    fn part_2_iter(
        &self,
        mut stack: Vec<String>,
        current: &str,
        twice_visited: bool,
    ) -> (usize, Vec<String>) {
        let mut result = 0;

        for next in self.paths.get(current).unwrap() {
            if next == "end" {
                result += 1;
                continue;
            }
            if next == "start" {
                continue;
            }
            let exists_in_stack = stack.contains(next) && !Caves::is_big(next);
            if exists_in_stack && twice_visited {
                continue;
            }

            stack.push(String::from(next));
            let (partial, next_stack) =
                self.part_2_iter(stack, next, twice_visited | exists_in_stack);
            stack = next_stack;
            stack.pop();
            result += partial;
        }

        (result, stack)
    }

    fn is_big(cave: &str) -> bool {
        cave.to_ascii_uppercase().eq(cave)
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let graph = Caves::new(lines).context("could not create graph")?;

    let ans1 = Ok(graph.part_1().to_string());
    let ans2 = Ok(graph.part_2().to_string());

    Ok((ans1, ans2))
}
