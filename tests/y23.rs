use aoc_rs::util::puzzles::puzzle_generator;
use aoc_rs::util::puzzles::Puzzle;

pub fn examples() -> Vec<Puzzle> {
    let puzzle = puzzle_generator(21);
    vec![
        puzzle(
            1,
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
            "142",
            "",
        ),
        puzzle(
            1,
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
            "",
            "281",
        ),
    ]
}
