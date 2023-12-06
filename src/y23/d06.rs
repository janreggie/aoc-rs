use crate::util::vectors::split_and_trim_borrowed;
use anyhow::{bail, Context, Result};

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn count_possible_ways_to_win(&self) -> u64 {
        // There is a closed-form solution to this.
        // Let x be the number of milliseconds to hold the button at the beginning.
        // Then, we should count the number of x's such that
        //   x*(time-x) > distance
        //   ==>  x^2-time*x+distance < 0
        // ...but for now, we'll just count it in linear time.
        (0..self.time)
            .step_by(1)
            .filter(|time| time * (self.time - time) > self.distance)
            .count() as u64
    }
}

#[test]
fn test_race_count_possible_ways_to_win() {
    let races = vec![
        Race { time: 7, distance: 9 },
        Race { time: 15, distance: 40 },
        Race { time: 30, distance: 200 },
    ];
    let answers = vec![4, 8, 9];
    for ii in 0..3 {
        assert_eq!(races[ii].count_possible_ways_to_win(), answers[ii]);
    }
}

struct Races {
    races: Vec<Race>,
}

impl Races {
    fn new(lines: Vec<String>) -> Result<Races> {
        if lines.len() != 2 {
            bail!("lines should be of length 2, got {} instead", lines.len())
        }

        let (times, distances) = (&lines[0], &lines[1]);
        let times = times
            .strip_prefix("Time: ")
            .and_then(|s| {
                split_and_trim_borrowed(s, ' ')
                    .into_iter()
                    .map(|t| t.parse::<u64>().ok())
                    .collect::<Option<Vec<_>>>()
            })
            .context("cannot parse times")?;
        let distances = distances
            .strip_prefix("Distance: ")
            .and_then(|s| {
                split_and_trim_borrowed(s, ' ')
                    .into_iter()
                    .map(|t| t.parse::<u64>().ok())
                    .collect::<Option<Vec<_>>>()
            })
            .context("cannot parse distances")?;
        if times.len() != distances.len() {
            bail!("length of times not equal to length of distances");
        }

        let races = times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| Race { time, distance })
            .collect();
        Ok(Races { races })
    }

    fn get_possible_ways_to_win(&self) -> Vec<u64> {
        self.races
            .iter()
            .map(|race| race.count_possible_ways_to_win())
            .collect()
    }

    fn combine_all_races(&self) -> Race {
        let (times, distances): (Vec<_>, Vec<_>) = self
            .races
            .iter()
            .map(|race| (race.time.to_string(), race.distance.to_string()))
            .unzip();
        let time =
            times.into_iter().collect::<String>().parse::<u64>().unwrap();
        let distance =
            distances.into_iter().collect::<String>().parse::<u64>().unwrap();

        Race { time, distance }
    }
}

fn solve_part_1(races: &Races) -> Result<String> {
    Ok(races.get_possible_ways_to_win().iter().product::<u64>().to_string())
}

fn solve_part_2(races: &Races) -> Result<String> {
    Ok(races.combine_all_races().count_possible_ways_to_win().to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let races = Races::new(lines).context("could not parse input")?;
    Ok((solve_part_1(&races), solve_part_2(&races)))
}
