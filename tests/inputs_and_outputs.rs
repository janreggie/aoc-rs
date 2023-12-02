use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use aoc_rs::util::{self, puzzles::Puzzle};
use sscanf::sscanf;

/// Reads input filename and output filename. Assume that output_filename contains at least two lines.
fn read_puzzle_from_files(
    year: u8,
    day: u8,
    input_file: &PathBuf,
    output_file: &PathBuf,
) -> Result<Puzzle> {
    let input_data = fs::read_to_string(input_file).with_context(|| {
        format!("could not read input file {}", input_file.to_str().unwrap())
    })?;
    let answers = fs::read_to_string(output_file).with_context(|| {
        format!("could not read output file {}", output_file.to_str().unwrap())
    })?;
    let answers = answers.split('\n').collect::<Vec<_>>();
    if answers.len() < 2 {
        bail!("output file {} too short", output_file.to_str().unwrap());
    }
    let (answer_a, answer_b) = (answers[0], answers[1]);
    let answer_a =
        if answer_a.is_empty() { None } else { Some(answer_a.to_string()) };
    let answer_b =
        if answer_b.is_empty() { None } else { Some(answer_b.to_string()) };

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
    // Navigate through the folder
    let root_folder = Path::new(env!("CARGO_MANIFEST_DIR"));
    let inputs_folder = root_folder.join("tests/inputs");
    let outputs_folder = root_folder.join("tests/outputs");

    let input_paths = inputs_folder.read_dir().unwrap_or_else(|_| {
        panic!(
            "could not read input folder {}",
            inputs_folder.to_str().unwrap()
        )
    });

    for path in input_paths {
        let path = path.unwrap();
        let input_file = path.path();
        let file_name = path.file_name();
        let (year, day, _) = sscanf!(
            file_name.to_str().unwrap(),
            "y{}-d{}{:/.+/}",
            u8,
            u8,
            &str
        )
        .unwrap();
        let output_file = outputs_folder.join(file_name);

        let puzzle =
            read_puzzle_from_files(year, day, &input_file, &output_file)
                .unwrap();
        util::test_puzzle(&puzzle);
    }
}
