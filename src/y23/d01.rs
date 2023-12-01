use std::collections::HashMap;

use anyhow::{Context, Result};

/// Get the first digit for the calibration value.
fn get_first_calibration_digit(line: &str, use_words: bool) -> Option<u32> {
    if !use_words {
        return line.chars().find_map(|c| c.to_digit(10));
    }

    let to_look_for = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);

    let mut all_indexes = to_look_for
        .iter()
        .filter_map(|(key, value)| {
            line.find(key).and_then(|index| Some((index, *value)))
        })
        .collect::<Vec<_>>();
    all_indexes.sort_by_key(|(index, _)| *index);
    all_indexes.first().map(|(_index, value)| *value)
}

/// Get the last digit for the calibration value.
fn get_last_calibration_digit(line: &str, use_words: bool) -> Option<u32> {
    if !use_words {
        return line.chars().rev().find_map(|c| c.to_digit(10));
    }

    let to_look_for = HashMap::from([
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ]);

    let mut all_indexes = to_look_for
        .iter()
        .filter_map(|(key, value)| {
            line.rfind(key).and_then(|index| Some((index, *value)))
        })
        .collect::<Vec<_>>();
    all_indexes.sort_by_key(|(index, _)| *index);
    all_indexes.last().map(|(_index, value)| *value)
}

/// Get the calibration value.
fn get_calibration_value(line: &str, use_words: bool) -> Option<u32> {
    let first_digit = get_first_calibration_digit(line, use_words);
    let last_digit = get_last_calibration_digit(line, use_words);

    match (first_digit, last_digit) {
        (Some(l), Some(r)) => Some(l * 10 + r),
        _ => None,
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    // Part 1: Only consider digits
    let calibration_values = lines
        .iter()
        .map(|line| {
            get_calibration_value(line, false).with_context(|| {
                format!("string {} does not contain words", line)
            })
        })
        .collect::<Result<Vec<u32>>>()?;
    let ans1 = calibration_values.iter().sum::<u32>().to_string();

    // Part 2: Now, consider the strings
    let calibration_values = lines
        .iter()
        .map(|line| {
            get_calibration_value(line, true).with_context(|| {
                format!("string {} does not contain words", line)
            })
        })
        .collect::<Result<Vec<u32>>>()?;
    let ans2 = calibration_values.iter().sum::<u32>().to_string();

    Ok((ans1, ans2))
}
