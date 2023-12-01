use crate::util::vectors;
use anyhow::Result;

/// Sorts a string by chars ("acdb" -> "abcd")
fn sort_string(s: &String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let passphrases = lines;

    // Number of valid passphrases for parts 1 and 2 respectively
    let mut valid_passphrases_1 = 0;
    let mut valid_passphrases_2 = 0;

    for passphrase in passphrases {
        let mut split_passphrase = vectors::split_and_trim(&passphrase, ' ');
        let initial_length = split_passphrase.len();

        // Part 1: Check if the passphrase uses duplicate words
        split_passphrase.sort();
        split_passphrase.dedup();
        let final_length = split_passphrase.len();
        if initial_length == final_length {
            valid_passphrases_1 += 1;
        }

        // Part 2: Check if the passphrase uses anagrams of each other
        split_passphrase = split_passphrase.iter().map(sort_string).collect();
        split_passphrase.sort();
        split_passphrase.dedup();
        let final_length = split_passphrase.len();
        if initial_length == final_length {
            valid_passphrases_2 += 1;
        }
    }

    let ans1 = Ok(valid_passphrases_1.to_string());
    let ans2 = Ok(valid_passphrases_2.to_string());

    Ok((ans1, ans2))
}
