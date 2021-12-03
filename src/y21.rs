mod d01;
mod d02;
mod d03;

pub fn solver(day: u8) -> fn(&Vec<String>) -> Result<(String, String), String> {
    match day {
        1 => d01::d01,
        2 => d02::d02,
        3 => d03::d03,
        _ => panic!("unimplemented day {}", day),
    }
}
