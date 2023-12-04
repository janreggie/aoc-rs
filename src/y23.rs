use anyhow::Result;

mod d01;
mod d02;
mod d03;
mod d04;

pub fn solver(
    day: u8,
) -> fn(Vec<String>) -> Result<(Result<String>, Result<String>)> {
    match day {
        1 => d01::solve,
        2 => d02::solve,
        3 => d03::solve,
        4 => d04::solve,
        _ => todo!("day {}", day),
    }
}
