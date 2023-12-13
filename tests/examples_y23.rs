use aoc_rs::util::puzzles::puzzle_generator;
use aoc_rs::util::puzzles::Puzzle;
use aoc_rs::util::test_puzzle;

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
        puzzle(
            4,
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            "13",
            "30",
        ),
        puzzle(
            5,
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
            "35",
            "46",
        ),
        puzzle(
            6,
            "Time:      7  15   30
Distance:  9  40  200",
            "288",
            "71503",
        ),
        puzzle(
            7,
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
            "6440",
            "5905",
        ),
        puzzle(
            8,
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
            "2",
            "",
        ),
        puzzle(
            8,
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
            "6",
            "",
        ),
        puzzle(
            8,
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
            "",
            "6",
        ),
        puzzle(
            9,
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
            "114",
            "2",
        ),
        puzzle(
            10,
            ".....
.S-7.
.|.|.
.L-J.
.....",
            "4",
            "",
        ),
        puzzle(
            10,
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
            "8",
            "",
        ),
        puzzle(
            10,
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
            "",
            "4",
        ),
        puzzle(
            10,
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
            "",
            "8",
        ),
        puzzle(
            10,
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
            "",
            "10",
        ),
        puzzle(
            11,
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            "374",
            "",
        ),
        puzzle(
            12,
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
            "21",
            "525152",
        ),
        puzzle(
            13,
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
            "405",
            "400",
        ),
    ]
}

#[test]
fn test_examples() {
    let puzzles = examples();
    for puzzle in &puzzles {
        test_puzzle(puzzle);
    }
}
