use anyhow::{bail, Context, Result};

#[derive(Debug)]
enum Snailfish {
    Number(u128),
    Pair(Box<Snailfish>, Box<Snailfish>),
}

impl Snailfish {
    /// Returns a Snailfish::Pair, or tries to.
    fn new(input: &str) -> Result<Snailfish> {
        /// Iterative constructor, returns the Snailfish that it first sees and the remaining characters
        fn it(mut input: &[char]) -> Result<(Snailfish, &[char])> {
            if input.len() == 0 {
                bail!("input cannot be empty")
            }

            // If the input is a number...
            if input[0].is_numeric() {
                let mut ff = 0;
                while input.len() > 0 && input[0].is_digit(10) {
                    ff *= 10;
                    ff += input[0].to_digit(10).unwrap() as u128;
                    input = &input[1..];
                }
                return Ok((Snailfish::Number(ff), input));
            }

            // The starting character
            let start = input[0];
            if start != '[' {
                bail!("expected start character `[`, got `{}` instead", start)
            }
            input = &input[1..];

            // The first term in our Snailfish
            if input.len() == 0 {
                bail!("premature exit before first term")
            }
            let first: Snailfish;
            let (ff, ip) = it(&input[0..]).context("could not extract first term")?;
            first = ff;
            input = ip;

            // Interlude: should have a comma separator
            if input.len() == 0 {
                bail!("premature exit after first term")
            } else if input[0] != ',' {
                bail!("expected comma separator, got `{}` instead", input[0])
            }
            input = &input[1..];

            // The second term in our Snailfish
            if input.len() == 0 {
                bail!("premature exit before second term")
            }
            let second: Snailfish;
            let (ff, ip) = it(&input[0..]).context("could not extract second term")?;
            second = ff;
            input = ip;

            // The ending character
            if input.len() == 0 {
                bail!("premature exit after second term")
            } else if input[0] != ']' {
                bail!("expected exit character `]`, got `{}` insetad", input[0])
            }
            input = &input[1..];

            Ok((Snailfish::Pair(Box::new(first), Box::new(second)), &input))
        }

        let input: Vec<char> = input.chars().collect();
        let (res, rem) = it(&input).context("could not create snailfish")?;
        if rem.len() != 0 {
            bail!(
                "there are still {} unparsed characters: `{:?}`",
                rem.len(),
                rem
            )
        }
        Ok(res)
    }

    fn add(self, other: Snailfish) -> Snailfish {
        Snailfish::Pair(Box::new(self), Box::new(other)).simplify()
    }

    fn simplify(mut self) -> Snailfish {
        // TODO: Try exploding

        // TODO: Try splitting

        // And once we're done...
        self
    }

    fn magnitude(&self) -> u128 {
        match self {
            Snailfish::Number(x) => *x,
            Snailfish::Pair(p1, p2) => p1.magnitude() + 2 * p2.magnitude(),
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut snailfishes = Vec::new();
    for line in lines {
        let snailfish =
            Snailfish::new(&line).context(format!("could not parse line `{}`", line))?;
        dbg!(&snailfish);
        snailfishes.push(snailfish);
    }

    unimplemented!()
}
