mod d01;
mod d02;
mod d03;
mod d04;
mod d05;

pub fn solver(day: u8) -> fn(Vec<String>) -> Result<(String, String), String> {
    match day {
        1 => d01::d01,
        2 => d02::d02,
        3 => d03::d03,
        4 => d04::d04,
        5 => d05::d05,
        _ => panic!("unimplemented day {}", day),
    }
}
