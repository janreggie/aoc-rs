use std::collections::HashMap;

use anyhow::{bail, Context, Result};

fn byte_to_usize(b: u8) -> usize {
    (b - b'a').into()
}

/// Returns ind such that bytes[ind-length:ind] contains unique characters, if such ind exists.
/// If bytes is shorter than length, return None.
/// It is assumed that bytes contains lowercase letters 'a' to 'z'.
fn get_unique_span(bytes: &[u8], length: usize) -> Option<usize> {
    if bytes.len() < length {
        return None;
    }

    // Initial population
    let mut marked_bytes = [false; 26];
    let mut duplicate_bytes: HashMap<u8, i32> = HashMap::new(); // number of excess times byte appears
    for ii in 0..length {
        let b = bytes[ii];
        if marked_bytes[byte_to_usize(b)] {
            duplicate_bytes.entry(b).and_modify(|c| *c += 1).or_insert(1);
        }
        marked_bytes[byte_to_usize(b)] = true;
    }

    // Now, span
    for ii in length..bytes.len() {
        let (prev_byte, next_byte) = (bytes[ii - length], bytes[ii]);
        // First, either remove prev_byte
        if duplicate_bytes.contains_key(&prev_byte) {
            duplicate_bytes.entry(prev_byte).and_modify(|c| *c -= 1);
            if *duplicate_bytes.get(&prev_byte).unwrap_or(&0) == 0 {
                duplicate_bytes.remove(&prev_byte);
            }
        } else {
            marked_bytes[byte_to_usize(prev_byte)] = false;
        }

        // Then, insert next_byte
        if marked_bytes[byte_to_usize(next_byte)] {
            duplicate_bytes
                .entry(next_byte)
                .and_modify(|c| *c += 1)
                .or_insert(1);
        } else {
            marked_bytes[byte_to_usize(next_byte)] = true;
        }

        // Finally, check if there are no longer any duplicate bytes
        if duplicate_bytes.len() == 0 {
            return Some(ii + 1); // Explain why the +1?!
        }
    }

    None
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expected input to be of length 1, got {}", lines.len())
    }
    let input = lines.into_iter().next().unwrap();
    let input = input.as_bytes();
    if input.len() < 14 {
        bail!("input too short")
    }

    let ans1 = Ok(get_unique_span(input, 4)
        .context("could not get start of packet marker")?
        .to_string());
    let ans2 = Ok(get_unique_span(input, 14)
        .context("could not get start of message marker")?
        .to_string());

    Ok((ans1, ans2))
}
