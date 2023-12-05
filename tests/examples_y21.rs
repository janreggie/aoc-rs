use aoc_rs::util::puzzles::puzzle_generator;
use aoc_rs::util::puzzles::Puzzle;
use aoc_rs::util::test_puzzle;

pub fn examples() -> Vec<Puzzle> {
    let puzzle = puzzle_generator(21);
    vec![
        puzzle(1, "199
200
208
210
200
207
240
269
260
263", "7", "5"),
puzzle(2, "forward 5
down 5
forward 8
up 3
down 8
forward 2", "150", "900"),
puzzle(3, "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010", "198", "230"),
puzzle(4, "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7", "4512", "1924"),
puzzle(5, "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2", "5", "12"),
puzzle(6, "3,4,3,1,2", "5934", "26984457539"),
puzzle(7, "16,1,2,0,4,2,7,1,2,14", "37", "168"),
puzzle(8, "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", "0", "5353"),
puzzle(8, "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", "26", "61229"),
puzzle(9, "2199943210
3987894921
9856789892
8767896789
9899965678", "15", "1134"),
puzzle(10, "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]", "26397", "288957"),
puzzle(11, "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526", "1656", "195"),
puzzle(12, "start-A
start-b
A-c
A-b
b-d
A-end
b-end", "10", "36"),
puzzle(12, "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc", "19", "103"),
puzzle(12, "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW", "226", "3509"),
puzzle(13, "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5", "17", ""),
puzzle(14, "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C", "1588", "2188189693529"),
puzzle(15, "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581", "40", "315"),
puzzle(16, "D2FE28", "6", ""),
puzzle(16, "38006F45291200", "9", ""),
puzzle(16, "EE00D40C823060", "14", ""),
puzzle(16, "8A004A801A8002F478", "16", ""),
puzzle(16, "620080001611562C8802118E34", "12", ""),
puzzle(16, "C0015000016115A2E0802F182340", "23", ""),
puzzle(16, "A0016C880162017C3686B18A3D4780", "31", ""),
puzzle(16, "C200B40A82", "", "3"),
puzzle(16, "04005AC33890", "", "54"),
puzzle(16, "880086C3E88112", "", "7"),
puzzle(16, "CE00C43D881120", "", "9"),
puzzle(16, "D8005AC2A8F0", "", "1"),
puzzle(16, "F600BC2D8F", "", "0"),
puzzle(16, "9C005AC2F8F0", "", "0"),
puzzle(16, "9C0141080250320F1802104A08", "", "1"),
puzzle(17, "target area: x=20..30, y=-10..-5", "45", "112"),
puzzle(20, "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###", "35", "3351"),
puzzle(21, "Player 1 starting position: 4
Player 2 starting position: 8", "739785", "444356092776315"),
    ]
}

#[test]
fn test_examples() {
    let puzzles = examples();
    for puzzle in &puzzles {
        test_puzzle(puzzle);
    }
}
