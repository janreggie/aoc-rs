use anyhow::Result;

mod d01;

pub fn solver(
    day: u8,
) -> fn(Vec<String>) -> Result<(Result<String>, Result<String>)> {
    match day {
        1 => d01::solve,
        _ => todo!("day {}", day),
    }
}