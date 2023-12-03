use aoc_rs::util::puzzles::puzzle_generator;
use aoc_rs::util::puzzles::Puzzle;

pub fn examples() -> Vec<Puzzle> {
    let puzzle = puzzle_generator(23);
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
        puzzle(
            2,
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            "8",
            "",
        ),
        puzzle(
            3,
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            "4361",
            "467835",
        ),
    ]
}
