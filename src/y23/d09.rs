use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug)]
struct History {
    nums: Vec<i64>,
    // // coeffs[0]*x^0 + coeffs[1]*x^1 + ... == nums[x]
    // coeffs: Vec<i64>,
}

impl History {
    fn new(input: &str) -> Result<History> {
        let nums = input
            .split(' ')
            .map(|s| s.parse::<i64>().ok())
            .collect::<Option<Vec<_>>>()
            .context("cannot parse numbers properly")?;
        Ok(History { nums })
    }

    fn next(&self) -> i64 {
        Self::next_from(&self.nums)
    }

    fn next_from(nums: &Vec<i64>) -> i64 {
        if nums.iter().all_equal() {
            return nums[0];
        }
        // First, evaluate the differences between the numbers
        let diffs =
            nums.iter().tuple_windows().map(|(a, b)| b - a).collect::<Vec<_>>();
        let next_diff = Self::next_from(&diffs);
        nums.last().unwrap() + next_diff
    }

    fn prev(&self) -> i64 {
        let mut nums = self.nums.clone();
        nums.reverse();
        Self::next_from(&nums)
    }
}

#[test]
fn test_next_from_history() {
    let history = History::new("0 3 6 9 12 15").unwrap();
    assert_eq!(history.next(), 18);
    let history = History::new("1 3 6 10 15 21").unwrap();
    assert_eq!(history.next(), 28);
    let history = History::new("10 13 16 21 30 45").unwrap();
    assert_eq!(history.next(), 68);
}

fn solve_part_1(histories: &Vec<History>) -> Result<String> {
    Ok(histories.iter().map(|history| history.next()).sum::<i64>().to_string())
}

fn solve_part_2(histories: &Vec<History>) -> Result<String> {
    Ok(histories.iter().map(|history| history.prev()).sum::<i64>().to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let histories = lines
        .iter()
        .map(|line| {
            History::new(line)
                .context(format!("could not parse line `{}`", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("cannot parse input")?;
    Ok((solve_part_1(&histories), solve_part_2(&histories)))
}
