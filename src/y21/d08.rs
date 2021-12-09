use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Represents a set of seven signals from `a` to `g`.
#[derive(Copy, Clone)]
struct Signals {
    signals: [bool; 7],
}

impl Signals {
    /// Constructor.
    ///
    ///     new("abef") == new("fbae") == Signals([True,T,False,F,T,T,F])
    ///
    fn new(input: &str) -> Result<Signals> {
        let mut signals = [false; 7];
        for ch in input.chars() {
            let ind = match ch {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                x => bail!("could not parse char `{}`", x),
            };
            if signals[ind] {
                bail!("char `{}` appeared twice", ch);
            }
            signals[ind] = true;
        }
        Ok(Signals { signals })
    }

    /// Returns how many signals are "active".
    fn count_active(&self) -> usize {
        self.signals.iter().filter(|&v| *v).count()
    }

    /// Returns a new Signal based on a Mapping
    fn map(&self, mapping: &Mapping) -> Signals {
        // TODO: Implement
        panic!("unimplemented")
    }
}

impl BitAnd for Signals {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut signals = [false; 7];
        for ii in 0..7 {
            signals[ii] = self.signals[ii] & rhs.signals[ii];
        }
        Signals { signals }
    }
}

impl BitOr for Signals {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut signals = [false; 7];
        for ii in 0..7 {
            signals[ii] = self.signals[ii] | rhs.signals[ii];
        }
        Signals { signals }
    }
}

impl BitXor for Signals {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut signals = [false; 7];
        for ii in 0..7 {
            signals[ii] = self.signals[ii] ^ rhs.signals[ii];
        }
        Signals { signals }
    }
}

impl Not for Signals {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut signals = [false; 7];
        for ii in 0..7 {
            signals[ii] = !self.signals[ii];
        }
        Signals { signals }
    }
}

/// Represents a mapping from the erroneous Signals to their expected values.
/// For example, `"abcdefg" -> "acbfedg"` will have a Mapping of `[0,2,1,5,4,3,6]`,
/// because the character 'a' gets mapped to 'a' (`map[0] == 0`), 'b' to 'c' (`map[1] == 2`), 'f' to 'd' (`map[5] == 3`), etc.
struct Mapping {
    map: [usize; 7],
}

impl Mapping {
    fn new(patterns: &[Signals]) -> Result<Mapping> {
        // Do things here
        bail!("unimplemented")
    }
}

struct Entry {
    /// Left of `|`
    unique_patterns: Vec<Signals>,

    /// Right of `|`
    output_signals: Vec<Signals>,

    mapping: Mapping,
}

impl Entry {
    fn new(input: &str) -> Result<Entry> {
        let mut sides = vectors::split_and_trim(input, '|');
        if sides.len() != 2 {
            bail!(
                "expects input to be split into two, got {} divisions",
                sides.len()
            );
        }
        let rhs = sides.pop().unwrap(); // last element gets popped first
        let lhs = sides.pop().unwrap();

        let lhs = vectors::split_and_trim(&lhs, ' ');
        if lhs.len() != 10 {
            bail!("expects lhs to be split into 10, got {} instead", lhs.len());
        }
        let mut unique_patterns = Vec::new();
        for sigs in lhs {
            let signals = Signals::new(&sigs).context(format!(
                "could not interpret `{}` properly as signals",
                sigs
            ))?;
            unique_patterns.push(signals);
        }
        unique_patterns
            .sort_unstable_by(|a, b| a.count_active().partial_cmp(&b.count_active()).unwrap());

        let rhs = vectors::split_and_trim(&rhs, ' ');
        if rhs.len() != 4 {
            bail!("expects rhs to be split into 4, got {} instead", rhs.len());
        }
        let mut output_signals = Vec::new();
        for sigs in rhs {
            let signals = Signals::new(&sigs).context(format!(
                "could not interpret `{}` properly as signals",
                sigs
            ))?;
            output_signals.push(signals);
        }

        let mapping = Mapping::new(&unique_patterns).context("could not create mapping")?;
        for ii in 0..4 {
            // TODO: Validate if the mapped output is valid
            output_signals[ii] = output_signals[ii].map(&mapping);
        }

        Ok(Entry {
            unique_patterns,
            output_signals,
            mapping,
        })
    }

    fn count_1478(&self) -> usize {
        self.output_signals
            .iter()
            .filter(|s| {
                let c = s.count_active();
                c == 2 || c == 4 || c == 3 || c == 7
            })
            .count()
    }

    /// Retrieves the four-digit output
    fn get_output(&self) -> u32 {
        // TODO: Implement
        panic!("unimplemented")
    }
}

pub fn d08(lines: Vec<String>) -> Result<(String, String)> {
    let mut entries = Vec::new();
    for line in lines {
        let entry = Entry::new(&line).context(format!("could not process line `{}`", line))?;
        entries.push(entry);
    }

    // Part 1: Count how many are 1, 4, 7, or 8
    let mut ans1 = 0;
    for entry in &entries {
        ans1 += entry.count_1478();
    }

    // Part 2: "Reading" the output values... this will be a bit more complex
    let mut ans2 = 0;
    for entry in &entries {
        ans2 += entry.get_output();
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
