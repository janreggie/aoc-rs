use crate::y17;
use crate::y21;
use crate::y22;
use crate::y23;
use anyhow::Result;

pub mod puzzles;
pub mod vectors;

/// Solver
pub fn solve(
    yr: u8,
) -> fn(u8) -> fn(Vec<String>) -> Result<(Result<String>, Result<String>)> {
    match yr {
        17 => y17::solver,
        21 => y21::solver,
        22 => y22::solver,
        23 => y23::solver,
        _ => todo!("year {}", yr),
    }
}

/// For testing
pub fn test_puzzle(puzzle: &puzzles::Puzzle) {
    let puzzles::Puzzle { year, day, input_data, answer_a, answer_b } = puzzle;
    eprintln!("Testing year {} day {} input {}", year, day, input_data);
    let input = input_data
        .trim_matches('\n')
        .split('\n')
        .map(|s| s.to_string())
        .collect();
    let (actual_a, actual_b) = solve(*year)(*day)(input).unwrap();
    match answer_a {
        Some(ans) => assert_eq!(*ans, actual_a.unwrap()),
        None => (),
    }
    match answer_b {
        Some(ans) => assert_eq!(*ans, actual_b.unwrap()),
        None => (),
    }
}
