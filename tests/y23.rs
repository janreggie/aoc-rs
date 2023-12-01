use aoc_rs::y23::solver;

#[test]
fn test_y23() {
    let testcases = vec![
        (
            1,
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
            "142",
            "",
        ),
    ];
    for (day, input, ans1, ans2) in testcases {
        test_day(day, input, ans1, ans2);
    }
}

fn test_day(day: u8, input: &str, ans1: &str, ans2: &str) {
    eprintln!("Testing day {}", day);
    let input = split(input);
    let (r1, r2) = solver(day)(input).unwrap();
    if ans1 != "" {
        eprintln!("Checking if ans1 matches expected");
        assert_eq!(ans1, r1);
    }
    if ans2 != "" {
        eprintln!("Checking if ans2 matches expected");
        assert_eq!(ans2, r2);
    }
}

fn split(str: &str) -> Vec<String> {
    str.trim_matches('\n')
        .split('\n')
        .map(|s| s.to_string())
        .collect()
}
