use aoc_rs::util::puzzles::puzzle_generator;
use aoc_rs::util::puzzles::Puzzle;

pub fn examples() -> Vec<Puzzle> {
    let puzzle = puzzle_generator(17);
    vec![
        puzzle(1, "1122", "3", ""),
        puzzle(1, "1111", "4", ""),
        puzzle(1, "1234", "0", ""),
        puzzle(1, "91212129", "9", ""),
        puzzle(1, "1212", "", "6"),
        puzzle(1, "1221", "", "0"),
        puzzle(1, "123425", "", "4"),
        puzzle(1, "123123", "", "12"),
        puzzle(1, "12131415", "", "4"),
        puzzle(
            2,
            "5 1 9 5
7 5 3
2 4 6 8",
            "18",
            "",
        ),
        puzzle(
            2,
            "5 9 2 8
9 4 7 3
3 8 6 5",
            "",
            "9",
        ),
        puzzle(3, "1", "0", ""),
        puzzle(3, "12", "3", ""),
        puzzle(3, "23", "2", ""),
        puzzle(3, "1024", "31", ""),
        puzzle(4, "aa bb cc dd ee", "1", ""),
        puzzle(4, "aa bb cc dd aa", "0", ""),
        puzzle(4, "aa bb cc dd aaa", "1", ""),
        puzzle(4, "abcde fghij", "", "1"),
        puzzle(4, "abcde xyz ecdab", "", "0"),
        puzzle(4, "a ab abc abd abf abj", "", "1"),
        puzzle(4, "iiii oiii ooii oooi oooo", "", "1"),
        puzzle(4, "oiii ioii iioi iiio", "", "0"),
        puzzle(
            5,
            "0
3
0
1
-3",
            "5",
            "10",
        ),
        puzzle(6, "0    2       7       0", "5", "4"),
        puzzle(
            7,
            "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)",
            "tknk",
            "60",
        ),
        puzzle(
            8,
            "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10",
            "1",
            "10",
        ),
        puzzle(9, "{}", "1", ""),
        puzzle(9, "{{{}}}", "6", ""),
        puzzle(9, "{{},{}}", "5", ""),
        puzzle(9, "{{{},{},{{}}}}", "16", ""),
        puzzle(9, "{<a>,<a>,<a>,<a>}", "1", ""),
        puzzle(9, "{{<ab>},{<ab>},{<ab>},{<ab>}}", "9", ""),
        puzzle(9, "{{<!!>},{<!!>},{<!!>},{<!!>}}", "9", ""),
        puzzle(9, "{{<a!>},{<a!>},{<a!>},{<ab>}}", "3", ""),
        puzzle(9, "{<>}", "", "0"),
        puzzle(9, "{<random characters>}", "", "17"),
        puzzle(9, "{<<<<>}", "", "3"),
        puzzle(9, "{<{!>}>}", "", "2"),
        puzzle(9, "{<!!>}", "", "0"),
        puzzle(9, "{<!!!>>}", "", "0"),
        puzzle(9, r#"{<{o"i!a,<{i<a>}"#, "", "10"),
        puzzle(10, "", "", "a2582a3a0e66e6e86e3812dcb672a272"),
        puzzle(10, "AoC 2017", "", "33efeb34ea91902bb2f59c9920caa6cd"),
        puzzle(10, "1,2,3", "", "3efbe78a8d82f29979031a4aa0b16a9d"),
        puzzle(10, "1,2,4", "", "63960835bcdc130f0b66d7ff4f6a5a8e"),
        puzzle(11, "ne,ne,ne", "3", ""),
        puzzle(11, "ne,ne,sw,sw", "0", ""),
        puzzle(11, "ne,ne,s,s", "2", ""),
        puzzle(11, "se,sw,se,sw,sw", "3", ""),
        puzzle(
            12,
            "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5",
            "6",
            "2",
        ),
        puzzle(
            13,
            "0: 3
1: 2
4: 4
6: 4",
            "24",
            "10",
        ),
        puzzle(14, "flqrgnkx", "8108", "1242"),
        puzzle(
            15,
            "Generator A starts with 65
Generator B starts with 8921",
            "588",
            "309",
        ),
        // puzzle(16, "s1,x3/4,pe/b", "baedc", "ceadb"), // TODO: Unit test
    ]
}
