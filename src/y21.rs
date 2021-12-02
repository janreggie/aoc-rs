mod d01;
mod d02;

pub fn solver(day: u8) -> fn(&Vec<String>) -> Result<(String, String), String> {
    match day {
        1 => d01::d01,
        2 => d02::d02,
        _ => panic!("unimplemented day {}", day),
    }
}
