use aoc_rs::util;
use clap::{App, Arg};
use std::io::{self, BufRead};
use std::ops::Sub;
use std::time::Instant;

fn main() {
    println!("Welcome to janreggie/aoc-rs");

    // Read flags
    let matches = App::new("aoc-rs")
        .author("Jan Reggie Dela Cruz")
        .about("Advent of Code in Rust")
        .arg(
            Arg::with_name("year")
                .short("y")
                .long("year")
                .takes_value(true)
                .value_name("yr")
                .help("Year to use (e.g., `21`)"),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .value_name("d")
                .help("Day to use (e.g., `5`)"),
        )
        .get_matches();
    let year: u8 = matches
        .value_of("year")
        .expect("year is required")
        .parse()
        .expect("year must be numeric");
    let day: u8 = matches
        .value_of("day")
        .expect("day is required")
        .parse()
        .expect("day must be numeric");
    let solver = util::series(year)(day);

    // Read stdin
    let lines = read_lines();
    let lines = lines.expect("error in reading stdin");

    // Do the needful
    let before = Instant::now();
    let result = solver(lines);
    let result = result.unwrap_or_else(|err| panic!("could not solve y{}-d{}: {}", year, day, err));
    let after = Instant::now();

    // Print out the results
    println!("Answer for Part 1 is {}", result.0);
    println!("Answer for Part 2 is {}", result.1);
    println!(
        "It took {:#?} to solve the current problem",
        after.sub(before)
    );
}

fn read_lines() -> io::Result<Vec<String>> {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut result = Vec::new();
    for line in lines {
        match line {
            Ok(l) => result.push(l),
            Err(e) => return Err(e),
        }
    }

    Ok(result)
}
