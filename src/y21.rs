mod d01;

pub fn solver(day: u8) -> fn(&Vec<String>) -> Result<(String, String), String> {
    match day {
        1 => d01::d01,
        _ => panic!("unimplemented day {}", day),
    }
}
