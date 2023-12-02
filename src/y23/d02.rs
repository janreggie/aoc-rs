use anyhow::{bail, Context, Result};

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn new(input: &str) -> Result<Cubes> {
        let split_input = input.split(", ");
        let (mut red, mut green, mut blue) = (0, 0, 0);

        for count in split_input {
            if let Some(rr) = count.strip_suffix(" red") {
                red += rr.parse::<u32>().context("could not parse reds")?;
            } else if let Some(gg) = count.strip_suffix(" green") {
                green += gg.parse::<u32>().context("could not parse greens")?;
            } else if let Some(bb) = count.strip_suffix(" blue") {
                blue += bb.parse::<u32>().context("could not parse blues")?;
            } else {
                bail!("unknown string {:?}", count)
            }
        }

        Ok(Cubes { red, green, blue })
    }
}

struct Game {
    index: u32,
    cubess: Vec<Cubes>,
}

impl Game {
    fn new(input: &str) -> Result<Game> {
        let split_input = input.split(": ").collect::<Vec<&str>>();
        if split_input.len() != 2 {
            bail!("cannot separate by colon");
        }

        let game_index = split_input[0]
            .strip_prefix("Game ")
            .and_then(|v| v.parse::<u32>().ok())
            .context("cannot parse game index")?;

        let cubess = split_input[1]
            .split("; ")
            .map(|raw_cubes| {
                Cubes::new(raw_cubes).with_context(|| {
                    format!("could not interpret cubes {:?}", raw_cubes)
                })
            })
            .collect::<Result<Vec<Cubes>>>()?;

        Ok(Game { index: game_index, cubess })
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let games = lines
        .iter()
        .map(|line| {
            Game::new(line)
                .with_context(|| format!("could not interpret line '{}'", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not parse input properly")?;

    // Part 1: Count how many games would be possible if the bag only contained 12 reds, 13 greens, 14 blues
    let mut ans1 = 0;
    'eachgame: for game in &games {
        for cubes in &game.cubess {
            if cubes.red > 12 || cubes.green > 13 || cubes.blue > 14 {
                continue 'eachgame;
            }
        }
        ans1 += game.index;
    }
    let ans1 = Ok(ans1.to_string());

    // Part 2: Evaluate the sum of "powers" of these cubes
    let mut ans2 = 0;
    for game in &games {
        let (mut most_reds, mut most_greens, mut most_blues) = (0, 0, 0);
        for cubes in &game.cubess {
            most_reds = most_reds.max(cubes.red);
            most_greens = most_greens.max(cubes.green);
            most_blues = most_blues.max(cubes.blue);
        }
        ans2 += most_reds * most_greens * most_blues;
    }
    let ans2 = Ok(ans2.to_string());

    Ok((ans1, ans2))
}
