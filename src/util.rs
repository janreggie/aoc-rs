use crate::y17;
use crate::y21;
use anyhow::Result;

pub mod vectors;

pub fn series(yr: u8) -> fn(u8) -> fn(Vec<String>) -> Result<(String, String)> {
    match yr {
        17 => y17::solver,
        21 => y21::solver,
        _ => todo!("year {}", yr),
    }
}
