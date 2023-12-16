use std::fmt::{Debug, Display};

use anyhow::{Context, Result};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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

impl Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Condition::Operational => '.',
            Condition::Damaged => '#',
            Condition::Unknown => '?',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug)]
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
    fn count_arrangements(&self) -> u64 {
        let condition_record_groups = &self
            .condition_records
            .split(|c| *c == Condition::Operational)
            .filter(|g| !g.is_empty())
            .collect::<Vec<_>>();
        self.count_arrangements_iter_grouped(
            &condition_record_groups,
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

    fn count_arrangements_iter_grouped(
        &self,
        condition_record_groups: &[&[Condition]],
        damaged_groups: &[usize],
    ) -> u64 {
        if condition_record_groups.len() == 1 {
            return self.count_arrangements_iter(
                condition_record_groups[0],
                damaged_groups,
            );
        }

        let (first_condition_records, remaining_condition_record_groups) =
            condition_record_groups.split_first().unwrap();
        let mut result = 0;
        for ii in 0..=damaged_groups.len() {
            let (damaged_groups_at_first, damaged_groups_at_rest) =
                damaged_groups.split_at(ii);
            let lower_bound = damaged_groups_at_first.iter().sum::<usize>()
                + damaged_groups_at_first.len();
            if lower_bound > first_condition_records.len() + 1 {
                break;
            }
            let first_condition_records_arrangements = self
                .count_arrangements_iter(
                    first_condition_records,
                    damaged_groups_at_first,
                );
            if first_condition_records_arrangements == 0 {
                continue;
            }
            result += first_condition_records_arrangements
                * self.count_arrangements_iter_grouped(
                    remaining_condition_record_groups,
                    damaged_groups_at_rest,
                );
        }
        result
    }

    fn count_arrangements_iter(
        &self,
        condition_records: &[Condition],
        damaged_groups: &[usize],
    ) -> u64 {
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

        // TODO: Optimize: if ends with #
        if condition_records.ends_with(&[Condition::Damaged]) {
            let (&last_damaged_group, rest_damaged_groups) =
                damaged_groups.split_last().unwrap();
            if condition_records[condition_records.len() - last_damaged_group..]
                .iter()
                .any(|f| *f == Condition::Operational)
            {
                return 0;
            }
            if condition_records.len() > last_damaged_group
                && condition_records
                    [condition_records.len() - last_damaged_group - 1]
                    == Condition::Damaged
            {
                return 0;
            }
            let next_condition_records =
                if condition_records.len() == last_damaged_group {
                    &condition_records
                        [..condition_records.len() - last_damaged_group]
                } else {
                    &condition_records
                        [..condition_records.len() - last_damaged_group - 1]
                };
            return self.count_arrangements_iter(
                next_condition_records,
                rest_damaged_groups,
            );
        }

        // TODO: Do some optimizations here... e.g., "if they're all ???'s"
        if condition_records.iter().all(|c| *c == Condition::Unknown) {
            let mut damaged_groups = damaged_groups.to_vec();
            damaged_groups.sort_by(|a, b| b.cmp(a));
            return self.count_arrangements_iter_if_all_unknowns(
                condition_records,
                &damaged_groups,
            );
        }

        // Check the first condition_record
        match condition_records[0] {
            Condition::Operational => self.count_arrangements_iter(
                &condition_records[1..],
                damaged_groups,
            ),
            Condition::Damaged => self
                .count_arrangements_iter_if_first_is_damaged(
                    condition_records,
                    damaged_groups,
                ),
            Condition::Unknown => {
                let f1 = self.count_arrangements_iter(
                    &condition_records[1..],
                    damaged_groups,
                );
                let f2 = self.count_arrangements_iter_if_first_is_damaged(
                    condition_records,
                    damaged_groups,
                );
                f1 + f2
            }
        }
    }

    fn count_arrangements_iter_if_all_unknowns(
        &self,
        condition_records: &[Condition],
        damaged_groups: &[usize],
    ) -> u64 {
        // Copyright (c) Stack Overflow, CC-BY-SA 4.0
        // <https://stackoverflow.com/a/65563202/14020202>
        fn count_combinations(n: u64, r: u64) -> u64 {
            if r > n {
                0
            } else {
                (1..=r.min(n - r)).fold(1, |acc, val| acc * (n - val + 1) / val)
            }
        }
        // Count how many Unknowns there would be left, minus considering all damaged groups.
        let damaged_count = damaged_groups.iter().sum::<usize>();
        // Then count how many ways are there to divvy the remaining Unknowns (which will be Operationals) to fit the continguous groups.
        let remaining_unknowns = condition_records.len() - damaged_count;
        count_combinations(
            remaining_unknowns as u64 + 1,
            damaged_groups.len() as u64,
        )
    }

    fn count_arrangements_iter_if_first_is_damaged(
        &self,
        condition_records: &[Condition],
        damaged_groups: &[usize],
    ) -> u64 {
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
        self.count_arrangements_iter(
            next_condition_records,
            &damaged_groups[1..],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_counts() {
        let cases = vec![
            ("???.### 1,1,3", 1),
            (".??..??...?##. 1,1,3", 4),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1),
            ("????.#...#... 4,1,1", 1),
            ("????.######..#####. 1,6,5", 4),
            ("?###???????? 3,2,1", 10),
        ];
        let cases =
            cases.into_iter().map(|(s, c)| (Springs::new(s).unwrap(), c));
        for (springs, count) in cases {
            println!("{:?} must have {:?} arrangements", &springs, count);
            assert_eq!(springs.count_arrangements(), count)
        }
    }

    #[test]
    fn test_spring_counts_all_unknowns() {
        let cases = vec![
            ("? 1", 1),
            ("??? 1", 3),
            ("?????? 2", 5),
            ("?????? 1,1", 10),
            ("?????? 1,2", 6),
        ];
        let cases =
            cases.into_iter().map(|(s, c)| (Springs::new(s).unwrap(), c));
        for (springs, count) in cases {
            println!("{:?} must have {:?} arrangements", &springs, count);
            assert_eq!(springs.count_arrangements(), count)
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
}

fn solve_part_1(all_springs: &Vec<Springs>) -> Result<String> {
    let mut result = 0;
    for springs in all_springs {
        let current_count = springs.count_arrangements();
        println!(
            "{} {}",
            Springs::pretty_print_condition_records_and_damaged_groups(
                &springs.condition_records,
                &springs.damaged_groups
            ),
            current_count
        );
        result += current_count;
    }
    Ok(result.to_string())
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
