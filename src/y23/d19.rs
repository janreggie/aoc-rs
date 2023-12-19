use anyhow::{bail, Context};
use itertools::{Group, Groups};
use sscanf::{sscanf, FromScanf};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    hash::BuildHasher,
    str::FromStr,
    thread::current,
};

use crate::util::vectors::group;

#[derive(PartialEq, Eq, Debug, FromScanf, Clone, Copy)]
enum Category {
    #[sscanf(format = "x")]
    X,
    #[sscanf(format = "m")]
    M,
    #[sscanf(format = "a")]
    A,
    #[sscanf(format = "s")]
    S,
}

#[derive(PartialEq, Eq, Debug, FromScanf)]
#[sscanf(format = "{{x={x},m={m},a={a},s={s}}}")]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

/// Range is left-inclusive i.e., Range(1,4001) means "numbers from 1 to 4000"
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Range(u32, u32);

impl Range {
    fn empty() -> Range {
        Range(0, 0)
    }

    fn is_empty(&self) -> bool {
        *self == Range::empty()
    }

    /// Splits a range in the middle with v.
    fn split(&self, v: u32) -> (Range, Range) {
        if v < self.0 {
            (Range::empty(), *self)
        } else if v >= self.1 {
            (*self, Range::empty())
        } else {
            (Range(self.0, v), Range(v, self.1))
        }
    }

    fn count(&self) -> u32 {
        self.1 - self.0
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    /// Splits PartRange via Category at v.
    /// The first value contains PartRange where at a given `category` is less than `v`,
    /// whereas the second value contains PartRange where at a given `category` is greater than or equal to `v`.
    fn split(
        &self,
        category: Category,
        v: u32,
    ) -> (Option<PartRange>, Option<PartRange>) {
        let range_to_split = match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        };
        let (left_range, right_range) = range_to_split.split(v);
        if left_range.is_empty() {
            (None, Some(self.clone()))
        } else if right_range.is_empty() {
            (Some(self.clone()), None)
        } else {
            match category {
                Category::X => (
                    Some(PartRange {
                        x: left_range,
                        m: self.m,
                        a: self.a,
                        s: self.s,
                    }),
                    Some(PartRange {
                        x: right_range,
                        m: self.m,
                        a: self.a,
                        s: self.s,
                    }),
                ),
                Category::M => (
                    Some(PartRange {
                        x: self.x,
                        m: left_range,
                        a: self.a,
                        s: self.s,
                    }),
                    Some(PartRange {
                        x: self.x,
                        m: right_range,
                        a: self.a,
                        s: self.s,
                    }),
                ),
                Category::A => (
                    Some(PartRange {
                        x: self.x,
                        m: self.m,
                        a: left_range,
                        s: self.s,
                    }),
                    Some(PartRange {
                        x: self.x,
                        m: self.m,
                        a: right_range,
                        s: self.s,
                    }),
                ),
                Category::S => (
                    Some(PartRange {
                        x: self.x,
                        m: self.m,
                        a: self.a,
                        s: left_range,
                    }),
                    Some(PartRange {
                        x: self.x,
                        m: self.m,
                        a: self.a,
                        s: right_range,
                    }),
                ),
            }
        }
    }

    fn count(&self) -> u64 {
        self.x.count() as u64
            * self.m.count() as u64
            * self.a.count() as u64
            * self.s.count() as u64
    }
}

#[derive(PartialEq, Eq, Debug, FromScanf, Clone)]
enum Destination {
    #[sscanf(format = "{:/[a-z]+/}")]
    WorkflowName(String),
    #[sscanf(format = "R")]
    Rejected,
    #[sscanf(format = "A")]
    Accepted,
}

#[derive(PartialEq, Eq, Debug, FromScanf, Clone, Copy)]
enum Operation {
    #[sscanf(format = ">")]
    GreaterThan,
    #[sscanf(format = "<")]
    LessThan,
}

#[derive(PartialEq, Eq, Debug, FromScanf, Clone)]
#[sscanf(format = "{}{}{}:{}")]
struct Condition(Category, Operation, u32, Destination);

impl Condition {
    /// Returns True if part matches condition
    fn check(&self, part: &Part) -> bool {
        let part_value = match self.0 {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        };
        match self.1 {
            Operation::GreaterThan => part_value > self.2,
            Operation::LessThan => part_value < self.2,
        }
    }

    /// Splits a PartRange between a PartRange that passes and a PartRange that fails the provided condition
    /// If one is empty, that means none of the provided PartRange is True or False.
    fn split(
        &self,
        part_range: &PartRange,
    ) -> (Option<PartRange>, Option<PartRange>) {
        let split_at = match self.1 {
            Operation::GreaterThan => self.2 + 1,
            Operation::LessThan => self.2,
        };
        let (left_part_range, right_part_range) =
            part_range.split(self.0, split_at);
        match self.1 {
            Operation::GreaterThan => (right_part_range, left_part_range), // reverse order since left is "less than split_at"
            Operation::LessThan => (left_part_range, right_part_range),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    fallback: Destination,
}

impl Workflow {
    fn check(&self, part: &Part) -> Destination {
        for condition in &self.conditions {
            if condition.check(part) {
                return condition.3.clone();
            }
        }

        self.fallback.clone()
    }

    /// Splits a part_range into multiple Destinations
    fn check_range(
        &self,
        part_range: &PartRange,
    ) -> Vec<(PartRange, Destination)> {
        let mut result = vec![];
        let mut remaining_part_range = part_range.clone();
        for condition in &self.conditions {
            let (matching_condition, not_matching_condition) =
                condition.split(&remaining_part_range);
            match (matching_condition, not_matching_condition) {
                (None, None) => {
                    panic!("invalid condition")
                }
                (None, Some(not_matching)) => {
                    remaining_part_range = not_matching;
                }
                (Some(matching), None) => {
                    result.push((matching, condition.3.clone()));
                    return result;
                }
                (Some(matching), Some(not_matching)) => {
                    result.push((matching, condition.3.clone()));
                    remaining_part_range = not_matching;
                }
            }
        }
        result.push((remaining_part_range, self.fallback.clone()));
        result
    }
}

impl FromStr for Workflow {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rules) = sscanf!(s, "{}{{{}}}", String, String)
            .ok()
            .context("cannot parse name and rules")?;

        let rules = rules.split(',').collect::<Vec<&str>>();
        let (fallback, conditions) =
            rules.split_last().context("rules is empty")?;
        let fallback = Destination::from_str(fallback)
            .ok()
            .context("cannot parse fallback destination")?;
        let conditions = conditions
            .iter()
            .map(|cond| {
                Condition::from_str(cond)
                    .ok()
                    .context(format!("cannot parse `{}` as condition", cond))
            })
            .collect::<anyhow::Result<Vec<_>>>()
            .context("cannot get conditions")?;

        Ok(Workflow { name, conditions, fallback })
    }
}

struct System {
    workflows: HashMap<String, Workflow>,
}

impl System {
    fn new(lines: &Vec<String>) -> anyhow::Result<System> {
        let workflows = lines
            .iter()
            .map(|line| {
                Workflow::from_str(&line)
                    .context(format!("cannot parse `{}` as workflow", line))
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let mut workflow_map = HashMap::new();
        for workflow in workflows {
            workflow_map.insert(workflow.name.clone(), workflow);
        }
        if !workflow_map.contains_key("in") {
            bail!("must contain key `in`");
        }
        Ok(System { workflows: workflow_map })
    }

    fn find_dest(&self, part: &Part) -> anyhow::Result<Destination> {
        let mut current_workflow = self.workflows.get("in").unwrap();
        loop {
            let next_destination = current_workflow.check(part);
            match next_destination {
                Destination::WorkflowName(next_workflow_name) => {
                    if let Some(next_workflow) =
                        self.workflows.get(&next_workflow_name)
                    {
                        current_workflow = next_workflow;
                    } else {
                        bail!(
                            "cannot find workflow name `{}`",
                            next_workflow_name
                        );
                    }
                }
                _ => return Ok(next_destination),
            }
        }
    }

    fn count_dest_range(&self, part_range: &PartRange) -> anyhow::Result<u64> {
        let mut result = 0;
        let mut queue = VecDeque::new();
        queue.push_back((
            part_range.clone(),
            Destination::WorkflowName("in".to_string()),
        ));

        while let Some((range, destination)) = queue.pop_front() {
            match destination {
                Destination::WorkflowName(workflow_name) => {
                    let workflow =
                        self.workflows.get(&workflow_name).context(format!(
                            "cannot find workflow with name {}",
                            workflow_name
                        ))?;
                    for next_pair in workflow.check_range(&range) {
                        queue.push_back(next_pair);
                    }
                }
                Destination::Rejected => {}
                Destination::Accepted => {
                    result += range.count();
                }
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsers() {
        assert_eq!(
            Workflow::from_str("spx{a<2006:qkq,m>2090:A,rfg}").unwrap(),
            Workflow {
                name: "spx".to_string(),
                conditions: vec![
                    Condition(
                        Category::A,
                        Operation::LessThan,
                        2006,
                        Destination::WorkflowName("qkq".to_string())
                    ),
                    Condition(
                        Category::M,
                        Operation::GreaterThan,
                        2090,
                        Destination::Accepted
                    )
                ],
                fallback: Destination::WorkflowName("rfg".to_string())
            }
        );

        assert_eq!(
            Part::from_str("{x=787,m=2655,a=1222,s=2876}").unwrap(),
            Part { x: 787, m: 2655, a: 1222, s: 2876 }
        )
    }
}

fn solve_part_1(system: &System, parts: &Vec<Part>) -> anyhow::Result<String> {
    let mut result = 0;
    for part in parts {
        let final_destination = system
            .find_dest(part)
            .context(format!("cannot find final destination for {:?}", part))?;
        match final_destination {
            Destination::Rejected => continue,
            Destination::Accepted => {
                result += (part.x + part.m + part.a + part.s) as u64
            }
            _ => bail!(
                "invalid final destination {:?} for part {:?}",
                final_destination,
                part
            ),
        }
    }

    Ok(result.to_string())
}

fn solve_part_2(system: &System) -> anyhow::Result<String> {
    let result = system.count_dest_range(&PartRange {
        x: Range(1, 4001),
        m: Range(1, 4001),
        a: Range(1, 4001),
        s: Range(1, 4001),
    })?;
    Ok(result.to_string())
}

pub fn solve(
    lines: Vec<String>,
) -> anyhow::Result<(anyhow::Result<String>, anyhow::Result<String>)> {
    let input = group(lines);
    if input.len() != 2 {
        bail!("expects input to be in 2 groups; got {}", input.len())
    }
    let (workflows, parts) = (&input[0], &input[1]);
    let system =
        System::new(workflows).context("cannot create system from input")?;
    let parts = parts
        .iter()
        .map(|p| {
            Part::from_str(p)
                .ok()
                .context(format!("cannot create part from {}", p))
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .context("cannot create parts from input")?;
    Ok((solve_part_1(&system, &parts), solve_part_2(&system)))
}
