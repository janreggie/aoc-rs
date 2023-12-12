use std::fmt::Display;

use anyhow::{bail, Context, Result};
use async_recursion::async_recursion;
use futures::executor::block_on;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl Condition {
    fn new(c: char) -> Option<Condition> {
        match c {
            '.' => Some(Condition::Operational),
            '#' => Some(Condition::Damaged),
            '?' => Some(Condition::Unknown),
            _ => None,
        }
    }

    fn new_from_string(s: &str) -> Option<Vec<Condition>> {
        s.chars().map(|c| Condition::new(c)).collect::<Option<Vec<_>>>()
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Condition::Operational => '.',
            Condition::Damaged => '#',
            Condition::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

struct Springs {
    condition_records: Vec<Condition>,
    damaged_groups: Vec<usize>,
}

impl Springs {
    fn new(input: &str) -> Option<Springs> {
        let split_input = input.split(' ').collect::<Vec<_>>();
        if split_input.len() != 2 {
            return None;
        }

        let condition_records = Condition::new_from_string(split_input[0])?;
        let damaged_groups = split_input[1]
            .split(',')
            .map(|n| n.parse::<usize>().ok())
            .collect::<Option<Vec<_>>>()?;

        Some(Springs { condition_records, damaged_groups })
    }

    fn unfold(&mut self) {
        let old_condition_records_len = self.condition_records.len();
        for _ in 0..4 {
            self.condition_records.push(Condition::Unknown);
            self.condition_records
                .extend_from_within(0..old_condition_records_len);
        }

        let old_damaged_groups_len = self.damaged_groups.len();
        for _ in 0..4 {
            self.damaged_groups.extend_from_within(0..old_damaged_groups_len);
        }
    }

    // Counts how many possible arrangements are there given condition_records
    fn count_arrangements(&self) -> u32 {
        Self::count_arrangements_iter(
            &self.condition_records,
            &self.damaged_groups,
        )
    }

    fn pretty_print_condition_records_and_damaged_groups(
        condition_records: &[Condition],
        damaged_groups: &[usize],
    ) -> String {
        let condition_records = condition_records
            .iter()
            .map(|c| format!("{}", c))
            .collect::<String>();
        let damaged_groups = damaged_groups
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join(",");
        format!("{} {}", condition_records, damaged_groups)
    }

    fn count_arrangements_iter(
        condition_records: &[Condition],
        damaged_groups: &[usize],
    ) -> u32 {
        // Base case when there are zero damaged groups
        if damaged_groups.len() == 0 {
            for cc in condition_records {
                if *cc == Condition::Damaged {
                    return 0;
                }
            }
            return 1;
        }

        // Base case when there are zero condition records
        if condition_records.len() == 0 {
            // damaged_groups.len() != 0
            return 0;
        }

        // Base case when it's impossible to match damaged_groups to condition_records
        if condition_records.len()
            < (damaged_groups.iter().sum::<usize>() + damaged_groups.len() - 1)
        {
            return 0;
        }

        // Check the first condition_record
        match condition_records[0] {
            Condition::Operational => Self::count_arrangements_iter(
                &condition_records[1..],
                damaged_groups,
            ),
            Condition::Damaged => {
                Self::count_arrangements_iter_if_first_is_damaged(
                    condition_records,
                    damaged_groups,
                )
            }
            Condition::Unknown => {
                let f1 = Self::count_arrangements_iter(
                    &condition_records[1..],
                    damaged_groups,
                );
                let f2 = Self::count_arrangements_iter_if_first_is_damaged(
                    condition_records,
                    damaged_groups,
                );
                f1 + f2
            }
        }
    }

    fn count_arrangements_iter_if_first_is_damaged(
        condition_records: &[Condition],
        damaged_groups: &[usize],
    ) -> u32 {
        // Check if it's possible to have the first damaged_groups[0] elements equal to something
        let first_damaged_group = damaged_groups[0];
        if condition_records.len() < first_damaged_group {
            return 0;
        }
        if condition_records.len() > first_damaged_group
            && condition_records[first_damaged_group] == Condition::Damaged
        {
            return 0;
        }
        for ii in 0..first_damaged_group {
            if condition_records[ii] == Condition::Operational {
                return 0;
            }
        }

        let next_condition_records =
            if condition_records.len() == first_damaged_group {
                &condition_records[first_damaged_group..]
            } else {
                &condition_records[first_damaged_group + 1..]
            };
        Self::count_arrangements_iter(
            next_condition_records,
            &damaged_groups[1..],
        )
    }
}

#[test]
fn test_spring_counts() {
    let all_springs = vec![
        Springs::new("???.### 1,1,3").unwrap(),
        Springs::new(".??..??...?##. 1,1,3").unwrap(),
        Springs::new("?#?#?#?#?#?#?#? 1,3,1,6").unwrap(),
        Springs::new("????.#...#... 4,1,1").unwrap(),
        Springs::new("????.######..#####. 1,6,5").unwrap(),
        Springs::new("?###???????? 3,2,1").unwrap(),
    ];
    let counts = vec![1, 4, 1, 1, 4, 10];
    for ii in 0..all_springs.len() {
        assert_eq!(all_springs[ii].count_arrangements(), counts[ii])
    }
}

#[test]
fn test_spring_counts_unfolded() {
    let test_cases = vec![
        (Springs::new("???.### 1,1,3").unwrap(), 1),
        (Springs::new(".??..??...?##. 1,1,3").unwrap(), 16384),
        (Springs::new("?#?#?#?#?#?#?#? 1,3,1,6").unwrap(), 1),
        (Springs::new("????.#...#... 4,1,1").unwrap(), 16),
        (Springs::new("????.######..#####. 1,6,5").unwrap(), 2500),
        (Springs::new("?###???????? 3,2,1").unwrap(), 506250),
    ];
    for (springs, count) in test_cases {
        let mut springs = springs;
        springs.unfold();
        assert_eq!(springs.count_arrangements(), count);
    }
}

fn solve_part_1(all_springs: &Vec<Springs>) -> Result<String> {
    Ok(all_springs
        .iter()
        .map(|springs| springs.count_arrangements())
        .sum::<u32>()
        .to_string())
}

fn solve_part_2(all_springs: Vec<Springs>) -> Result<String> {
    let mut result = 0;
    for mut springs in all_springs {
        springs.unfold();
        println!(
            "{}",
            Springs::pretty_print_condition_records_and_damaged_groups(
                &springs.condition_records,
                &springs.damaged_groups
            )
        );
        result += springs.count_arrangements();
    }
    Ok(result.to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let all_springs = lines
        .into_iter()
        .map(|line| {
            Springs::new(&line).context(format!("cannot parse line {}", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("cannot parse input")?;
    Ok((solve_part_1(&all_springs), solve_part_2(all_springs)))
}
