use crate::y21;

pub mod vectors;

pub fn series(yr: u8) -> fn(u8) -> fn(Vec<String>) -> Result<(String, String), String> {
    match yr {
        21 => y21::solver,
        _ => panic!("unimplemented year {}", yr),
    }
}
