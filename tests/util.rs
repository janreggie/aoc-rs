pub struct Puzzle {
    pub year: u8,
    pub day: u8,
    pub input_data: String,
    pub answer_a: Option<String>,
    pub answer_b: Option<String>,
}

/// Take in a year, and create a function to generate Puzzles in that given year.
/// This allows us to avoid having to use .to_string() repeatedly.
pub fn puzzle_generator(year: u8) -> impl Fn(u8, &str, &str, &str) -> Puzzle {
    move |day: u8, input_data: &str, answer_a: &str, answer_b: &str| Puzzle {
        year: year,
        day: day,
        input_data: input_data.to_string(),
        answer_a: {
            if answer_a.is_empty() {
                None
            } else {
                Some(answer_a.to_string())
            }
        },
        answer_b: {
            if answer_b.is_empty() {
                None
            } else {
                Some(answer_b.to_string())
            }
        },
    }
}
