use std::fs;

use anyhow::{bail, Context, Result};
use aoc_rs::util::puzzles::Puzzle;

/// Reads input filename and output filename. Assume that output_filename contains at least two lines.
fn read_puzzle_from_files(
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

#[test]
fn test_inputs_and_outputs() {
    // TODO: The needful
    ()
}
