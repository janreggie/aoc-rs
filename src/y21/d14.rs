use crate::util::vectors;
use anyhow::{bail, Context, Result};

#[derive(Debug)]
struct Polymers {
    // Consider the example NNCB.
    // The pairs here are "NN", "NC", and "CB".
    // If we let 'A' = 0, 'B' = 1, ...,
    // pairs[13][13] == pairs[13][2] == pairs[2][1] == 1.
    pairs: [[u128; 26]; 26],

    // first and last represent the first/last "characters" in the template.
    // Useful for determining the number of characters.
    first: usize,
    last: usize,

    // Insertion rules.
    // "CB -> H" is interpreted as rules[2][1] = 7,
    // since C,B,H -> 2,1,7.
    rules: [[Option<usize>; 26]; 26],
}

impl Polymers {
    fn new(lines: Vec<String>) -> Result<Polymers> {
        let mut groups = vectors::group(lines);
        if groups.len() != 2 {
            bail!(
                "could not group into 2, got {} groups instead",
                groups.len()
            );
        }
        let insertion_rules = groups.pop().unwrap();
        let mut template = groups.pop().unwrap();

        // Extract template
        if template.len() != 1 {
            bail!(
                "expected template to be only one line, got {}",
                template.len()
            );
        }
        let template = template.pop().unwrap();
        if template.len() < 1 {
            bail!("template somehow empty");
        }
        let template: Option<Vec<usize>> = template.chars().map(Polymers::parse_char).collect();
        let template = template.context("could not parse template")?;

        // Interpret template
        let mut pairs = [[0; 26]; 26];
        for ii in 0..template.len() - 1 {
            let (a, b) = (template[ii], template[ii + 1]);
            pairs[a][b] += 1;
        }
        let first = *template.first().unwrap();
        let last = *template.last().unwrap();

        // Extract insertion rules
        let mut rules = [[None; 26]; 26];
        for rule in insertion_rules {
            let ((a, b), r) = Polymers::parse_insertion_rule(&rule)
                .context(format!("could not interpret insertion rule `{}`", rule))?;
            if let Some(_) = rules[a][b] {
                bail!("rule `{}` conflicts with an earlier rule", rule);
            }
            rules[a][b] = Some(r);
        }

        Ok(Polymers {
            pairs,
            first,
            last,
            rules,
        })
    }

    /// 'A' -> 0, 'B' -> 1, ...
    /// Returns None if invalid char
    fn parse_char(c: char) -> Option<usize> {
        if c.is_ascii_uppercase() {
            Some((c.to_digit(36).unwrap() - 10) as usize)
        } else {
            None
        }
    }

    /// "CH -> B" returns ((2,1),7)
    fn parse_insertion_rule(s: &str) -> Option<((usize, usize), usize)> {
        let s: Vec<&str> = s.split(" -> ").collect();
        if s.len() != 2 {
            return None;
        }

        let first: Vec<char> = s[0].chars().collect();
        let last: Vec<char> = s[1].chars().collect();
        if first.len() != 2 || last.len() != 1 {
            return None;
        }

        let a = Polymers::parse_char(first[0])?;
        let b = Polymers::parse_char(first[1])?;
        let r = Polymers::parse_char(last[0])?;
        Some(((a, b), r))
    }

    /// Returns how many of 'A', 'B', ... are in the Polymers struct.
    fn counts(&self) -> [u128; 26] {
        let mut result = [0; 26];
        for a in 0..26 {
            for b in 0..26 {
                result[a] += self.pairs[a][b];
                result[b] += self.pairs[a][b];
            }
        }
        result[self.first] += 1;
        result[self.last] += 1;

        // Because we counted everything twice
        for ii in 0..26 {
            result[ii] /= 2;
        }
        result
    }

    /// "Iterates" insertion
    fn iter(&mut self) {
        let mut next_pairs = [[0; 26]; 26];
        for a in 0..26 {
            for b in 0..26 {
                if let Some(r) = self.rules[a][b] {
                    next_pairs[a][r] += self.pairs[a][b];
                    next_pairs[r][b] += self.pairs[a][b];
                }
            }
        }
        self.pairs = next_pairs;
    }

    fn highest_minus_lowest(&self) -> u128 {
        let mut lowest = 0;
        let mut highest = 0;
        for vv in self.counts() {
            if vv != 0 && (vv < lowest || lowest == 0) {
                lowest = vv;
            }
            if vv > highest {
                highest = vv;
            }
        }

        highest - lowest
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut polymers = Polymers::new(lines).context("could not create polymers struct")?;

    // Part 1: Count between the lowest and highest
    for _ in 0..10 {
        polymers.iter();
    }
    let ans1 = polymers.highest_minus_lowest();

    // Part 2: Let's run it 30 more times
    for _ in 0..30 {
        polymers.iter();
    }
    let ans2 = polymers.highest_minus_lowest();

    Ok((ans1.to_string(), ans2.to_string()))
}
