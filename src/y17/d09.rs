use anyhow::{bail, Context, Ok, Result};

#[derive(Debug)]
struct Garbage(String);

impl Garbage {
    /// Parses feed and returns Garbage
    fn parse(mut feed: &[u8]) -> Result<(Garbage, &[u8])> {
        match feed.first() {
            Some(b'<') => {
                feed = &feed[1..];
                let mut garbage = String::new();
                while let Some((first, rest)) = feed.split_first() {
                    if *first == b'>' {
                        return Ok((Garbage(garbage), rest));
                    }
                    if *first == b'!' {
                        (_, feed) = rest
                            .split_first()
                            .context("invalid exclamation point")?;
                        continue;
                    }
                    garbage.push(*first as char);
                    feed = rest;
                }
                bail!("feed consumed while parsing Garbage")
            }
            Some(ch) => bail!("expects '<', got '{}' instead", *ch as char),
            None => bail!("cannot parse empty Garbage"),
        }
    }
}

#[derive(Debug)]
struct Group {
    contents: Vec<Content>,
}

impl Group {
    /// Parses feed and returns Group
    fn parse(mut feed: &[u8]) -> Result<(Group, &[u8])> {
        match feed.first() {
            Some(b'{') => {
                feed = &feed[1..];
                let mut contents = vec![];
                while let Some((first, rest)) = feed.split_first() {
                    match first {
                        b'}' => {
                            feed = rest;
                            break;
                        }
                        b'<' => {
                            let (garbage, next_feed) = Garbage::parse(feed)
                                .context("could not parse feed")?;
                            feed = next_feed;
                            contents.push(Content::Garbage(garbage))
                        }
                        b'{' => {
                            let (group, next_feed) = Group::parse(feed)
                                .context("could not parse feed")?;
                            feed = next_feed;
                            contents.push(Content::Group(group))
                        }
                        b',' => {
                            feed = rest;
                        }
                        _ => bail!("unknown character {}", *first as char),
                    }
                }
                Ok((Group { contents }, feed))
            }
            Some(ch) => bail!(
                "expects '{{' to start group, got '{}' instead",
                *ch as char
            ),
            None => bail!("cannot parse empty Group"),
        }
    }

    /// Get the score
    fn score(&self) -> u32 {
        fn score_it(group: &Group, level: u32) -> u32 {
            let mut result = level;
            for content in &group.contents {
                if let Content::Group(g) = content {
                    result += score_it(g, level + 1);
                }
            }
            result
        }
        score_it(self, 1)
    }

    /// Get how much garbage there is
    fn garbage_count(&self) -> usize {
        self.contents
            .iter()
            .map(|cc| match cc {
                Content::Garbage(garbage) => garbage.0.len(),
                Content::Group(group) => group.garbage_count(),
            })
            .sum()
    }
}

#[derive(Debug)]
enum Content {
    Garbage(Garbage),
    Group(Group),
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expected only 1 line, got {}", lines.len())
    }
    let input = lines.iter().next().unwrap().as_bytes();
    let (group, bytes) =
        Group::parse(input).context("could not parse input properly")?;
    if !bytes.is_empty() {
        bail!("stray bytes {:?}", bytes);
    }

    // Part 1: Score in terms of groups
    let ans1 = Ok(group.score().to_string());

    // Part 2: Amount of garbage
    let ans2 = Ok(group.garbage_count().to_string());

    Ok((ans1, ans2))
}
