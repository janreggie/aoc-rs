use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Represents a set of seven signals from `a` to `g`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Signals {
    signals: [bool; 7],
}

const T: bool = true;
const F: bool = false;

const DIGITS: [[bool; 7]; 10] = [
    [T, T, T, F, T, T, T], // 0
    [F, F, T, F, F, T, F], // 1
    [T, F, T, T, T, F, T], // 2
    [T, F, T, T, F, T, T], // 3
    [F, T, T, T, F, T, F], // 4
    [T, T, F, T, F, T, T], // 5
    [T, T, F, T, T, T, T], // 6
    [T, F, T, F, F, T, F], // 7
    [T, T, T, T, T, T, T], // 8
    [T, T, T, T, F, T, T], // 9
];

impl Signals {
    /// Constructor.
    ///
    ///     new("abef") == new("fbae") == Signals([True,T,False,F,T,T,F])
    ///
    fn new(input: &str) -> Result<Signals> {
        let mut signals = [F; 7];
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
            signals[ind] = T;
        }
        Ok(Signals { signals })
    }

    /// Returns how many signals are "active".
    fn count_active(&self) -> usize {
        self.signals.iter().filter(|&v| *v).count()
    }

    /// Returns a new Signal based on a Mapping
    fn map(&self, mapping: &Mapping) -> Signals {
        let mut signals = [F; 7];
        for ii in 0..7 {
            signals[mapping.map[ii]] = self.signals[ii];
        }
        Signals { signals }
    }

    fn eq(&self, signals: [bool; 7]) -> bool {
        self.signals == signals
    }

    /// If there is only one (1) signal active, return the index to that signal.
    /// Otherwise, return nothing
    fn get_active_ind(&self) -> Option<usize> {
        let mut ind = 7; // something that is intentionally out of bounds
        for ii in 0..7 {
            if self.signals[ii] {
                if ind != 7 {
                    return None;
                }
                ind = ii;
            }
        }
        if ind == 7 {
            None
        } else {
            Some(ind)
        }
    }

    /// If the Signals represent a valid digit, return that number.
    /// Otherwise, return None.
    fn digit(&self) -> Option<u32> {
        for ii in 0..10 {
            if self.eq(DIGITS[ii]) {
                return Some(ii as u32);
            }
        }
        None
    }
}

impl BitAnd for Signals {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let mut signals = [F; 7];
        for ii in 0..7 {
            signals[ii] = self.signals[ii] & rhs.signals[ii];
        }
        Signals { signals }
    }
}

impl BitOr for Signals {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let mut signals = [F; 7];
        for ii in 0..7 {
            signals[ii] = self.signals[ii] | rhs.signals[ii];
        }
        Signals { signals }
    }
}

impl BitXor for Signals {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut signals = [F; 7];
        for ii in 0..7 {
            signals[ii] = self.signals[ii] ^ rhs.signals[ii];
        }
        Signals { signals }
    }
}

impl Not for Signals {
    type Output = Self;
    fn not(self) -> Self::Output {
        let mut signals = [F; 7];
        for ii in 0..7 {
            signals[ii] = !self.signals[ii];
        }
        Signals { signals }
    }
}

/// Represents a mapping from the erroneous Signals to their expected values.
/// For example, `"abcdefg" -> "acbfdeg"` will have a Mapping of `[0,2,1,4,5,3,6]`,
/// because the character 'a' gets mapped to 'a' (`map[0] == 0`), 'b' to 'c' (`map[1] == 2`), 'f' to 'e' (`map[5] == 3`), etc.
#[derive(Debug)]
struct Mapping {
    map: [usize; 7],
}

impl Mapping {
    fn new(patterns: &[Signals]) -> Result<Mapping> {
        if let Err(e) = Mapping::verify(patterns) {
            bail!(e.context("could not verify signals to create mapping"))
        }
        let mut map = [7; 7];

        // We know where digits 1, 7, 4, and 8 are located
        let digit_1 = patterns[0];
        let digit_7 = patterns[1];
        let digit_4 = patterns[2];

        // digit_1 ^ digit_7 to extract `a`
        let signals_a = digit_1 ^ digit_7;
        match signals_a.get_active_ind() {
            None => bail!("could not extract `a`"),
            Some(ind) => map[ind] = 0, // map to `a`
        }

        // Extract `g`.
        let digit_9_but_g = signals_a ^ digit_4;
        let mut ind_of_digit_9 = 0;
        for ii in 6..9 {
            // Look for a Patterns that only has one active digit when xor'd
            let pat = digit_9_but_g ^ patterns[ii];
            if let Some(ind) = pat.get_active_ind() {
                ind_of_digit_9 = ii;
                map[ind] = 6; // map that value to `g`
                break;
            }
        }
        if ind_of_digit_9 == 0 {
            bail!("could not find a pattern that maps to digit 9");
        }
        let digit_9 = patterns[ind_of_digit_9];

        // Hmm... How do we go from here though?
        let digit_0_but_b = !(digit_1 ^ digit_4);
        let mut ind_of_digit_0 = 0;
        for ii in 6..9 {
            let pat = digit_0_but_b ^ patterns[ii];
            if let Some(ind) = pat.get_active_ind() {
                ind_of_digit_0 = ii;
                map[ind] = 1; // map that value to `b`
                break;
            }
        }
        if ind_of_digit_0 == 0 {
            bail!("could not find a patern that matches to digit 0");
        }
        let digit_0 = patterns[ind_of_digit_0];
        let digit_6 = patterns[21 - ind_of_digit_9 - ind_of_digit_0]; // 6+7+8 == 21

        // Using 0, 1, 6, 9, we can map to `c`, `d`, `e`, and `f`
        let signals_c = !digit_6;
        match signals_c.get_active_ind() {
            None => bail!("could not map to signal `c`"),
            Some(ind) => map[ind] = 2, // map that value to `c`
        }
        let signals_d = !digit_0;
        match signals_d.get_active_ind() {
            None => bail!("could not map to signal `d`"),
            Some(ind) => map[ind] = 3, // map to `d`
        }
        let signals_e = !digit_9;
        match signals_e.get_active_ind() {
            None => bail!("could not map to signal `e`"),
            Some(ind) => map[ind] = 4, // map to `e`
        }
        let signals_f = digit_1 & digit_6;
        match signals_f.get_active_ind() {
            None => bail!("could not map to signal `f`"),
            Some(ind) => map[ind] = 5, // map to `f`
        }

        // Finally, validate our map
        let mut seen = [false; 7];
        for ii in 0..7 {
            let vv = map[ii];
            if vv == 7 {
                bail!("failed to put map[{}]", ii);
            }
            if seen[vv] {
                bail!("duplicate for map at {}", vv);
            }
            seen[vv] = true;
        }

        Ok(Mapping { map })
    }

    fn verify(patterns: &[Signals]) -> Result<()> {
        if patterns.len() != 10 {
            bail!(
                "expected patterns to be of length 10, got {} instead",
                patterns.len()
            )
        }
        let expected_actives = [2, 3, 4, 5, 5, 5, 6, 6, 6, 7];
        for ii in 0..10 {
            let expected = expected_actives[ii];
            let actual = patterns[ii].count_active();
            if expected != actual {
                bail!(
                    "expected that the {}'th Patterns be of length {}, got {} instead",
                    ii,
                    expected,
                    actual
                );
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Entry {
    /// Right of `|`
    output_signals: Vec<Signals>,
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
            output_signals[ii] = output_signals[ii].map(&mapping);
            if output_signals[ii].digit() == None {
                bail!("could not validate {}th digit in output", ii)
            }
        }

        Ok(Entry { output_signals })
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
    fn get_output(&self) -> Result<u32> {
        let mut result = 0;
        for ii in 0..4 {
            match self.output_signals[ii].digit() {
                None => bail!(
                    "could not parse {}th digit properly: {:?}",
                    ii,
                    self.output_signals[ii]
                ),
                Some(v) => result += v,
            }
            result *= 10;
        }
        result /= 10; // extra *=10 over there

        Ok(result)
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
        ans2 += entry
            .get_output()
            .context(format!("could not process entry {:?}", entry))?;
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
