use std::fs;

use anyhow::{bail, Context, Result};

pub struct Puzzle {
    pub year: u8,
    pub day: u8,
    pub input_data: String,
    pub answer_a: Option<String>,
    pub answer_b: Option<String>,
}

/// Reads input filename and output filename. Assume that output_filename contains at least two lines.
pub fn read_puzzle_from_files(
    year: u8,
    day: u8,
    input_filename: &str,
    output_filename: &str,
) -> Result<Puzzle> {
    let input_data = fs::read_to_string(input_filename).with_context(|| {
        format!("could not read input file {}", input_filename)
    })?;
    let answers = fs::read_to_string(output_filename).with_context(|| {
        format!("could not read output file {}", output_filename)
    })?;
    let answers = answers.split('\n').collect::<Vec<_>>();
    if answers.len() < 2 {
        bail!("output file {} too short", output_filename);
    }

    Ok(Puzzle {
        year,
        day,
        input_data,
        answer_a: Some(answers[0].to_string()),
        answer_b: Some(answers[1].to_string()),
    })
}

/// Take in a year, and create a function to generate Puzzles in that given year.
/// This allows us to avoid having to use .to_string() repeatedly.
pub fn puzzle_generator(year: u8) -> impl Fn(u8, &str, &str, &str) -> Puzzle {
    move |day: u8, input_data: &str, answer_a: &str, answer_b: &str| Puzzle {
        year: year,
        day: day,
        input_data: input_data.to_string(),
        answer_a: {
            if answer_a.is_empty() {
                None
            } else {
                Some(answer_a.to_string())
            }
        },
        answer_b: {
            if answer_b.is_empty() {
                None
            } else {
                Some(answer_b.to_string())
            }
        },
    }
}
