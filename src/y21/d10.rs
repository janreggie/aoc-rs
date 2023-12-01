use anyhow::{bail, Context, Result};

struct Punct(PType, PChar);

impl Punct {
    fn new(ch: char) -> Option<Punct> {
        match ch {
            '(' => Some(Punct(PType::Opening, PChar::Parenthesis)),
            ')' => Some(Punct(PType::Closing, PChar::Parenthesis)),
            '[' => Some(Punct(PType::Opening, PChar::Bracket)),
            ']' => Some(Punct(PType::Closing, PChar::Bracket)),
            '{' => Some(Punct(PType::Opening, PChar::Brace)),
            '}' => Some(Punct(PType::Closing, PChar::Brace)),
            '<' => Some(Punct(PType::Opening, PChar::Angle)),
            '>' => Some(Punct(PType::Closing, PChar::Angle)),
            _ => None,
        }
    }
}

enum PType {
    Opening,
    Closing,
}

#[derive(PartialEq, Eq)]
enum PChar {
    Parenthesis,
    Bracket,
    Brace,
    Angle,
}

impl PChar {
    fn corrupt_score(&self) -> u128 {
        match self {
            PChar::Parenthesis => 3,
            PChar::Bracket => 57,
            PChar::Brace => 1197,
            PChar::Angle => 25137,
        }
    }

    fn incomplete_score(&self) -> u128 {
        match self {
            PChar::Parenthesis => 1,
            PChar::Bracket => 2,
            PChar::Brace => 3,
            PChar::Angle => 4,
        }
    }
}

enum ChunkType {
    Corrupted,
    Incomplete,
}

fn syntax_error_score(line: &str) -> Result<(ChunkType, u128)> {
    let mut stack = Vec::new();
    for ch in line.chars() {
        let punct = Punct::new(ch);
        if let None = punct {
            bail!("invalid character `{}`", ch);
        }
        let punct = punct.unwrap();
        let Punct(t, c) = punct;
        match t {
            PType::Opening => stack.push(c),
            PType::Closing => {
                if stack.len() == 0 || stack.last().unwrap() != &c {
                    return Ok((ChunkType::Corrupted, c.corrupt_score()));
                }
                stack.pop();
            }
        }
    }

    // Finally, for an incomplete chunk
    let mut score = 0;
    while let Some(c) = stack.pop() {
        score *= 5;
        score += c.incomplete_score();
    }

    Ok((ChunkType::Incomplete, score))
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    // Two birds in one stone.
    // Or, two parts in one `for` loop.
    let mut ans1 = 0;
    let mut ans2_scores = Vec::new();
    for line in lines {
        match syntax_error_score(&line) {
            Ok((ChunkType::Corrupted, score)) => ans1 += score,
            Ok((ChunkType::Incomplete, score)) => ans2_scores.push(score),
            Err(e) => {
                bail!(e.context(format!("could not parse line `{}`", line)))
            }
        }
    }
    let ans1 = Ok(ans1.to_string());

    // Because ans2 requires the *middle* value
    ans2_scores.sort();
    let ans2 = ans2_scores
        .get(ans2_scores.len() / 2)
        .context("no incomplete scores")?;
    let ans2 = Ok(ans2.to_string());

    Ok((ans1, ans2))
}
