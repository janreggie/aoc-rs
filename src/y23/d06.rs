use crate::util::vectors::split_and_trim_borrowed;
use anyhow::{bail, Context, Result};
use num::integer::Roots;

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    /// Returns whether holding the button in `time` milliseconds will cause the player to win
    fn check_win(&self, time: u64) -> bool {
        time * (self.time - time) > self.distance
    }

    fn count_possible_ways_to_win(&self) -> u64 {
        // If time is small enough, doing extra work serves nobody.
        if self.time < 100 {
            return (0..=self.time)
                .step_by(1)
                .filter(|time| self.check_win(*time))
                .count() as u64;
        }

        // There is a closed-form solution to this.
        // Let x be the number of milliseconds to hold the button at the beginning.
        // Then, we should count the number of x's such that
        //   x*(time-x) > distance
        //   ==>  x^2-time*x+distance < 0
        let (a, b, c) = (1 as i64, -(self.time as i64), self.distance as i64);
        let determinant = b * b - 4 * a * c;
        if determinant <= 0 {
            return 0;
        }
        let (x_1, x_2) = (
            ((-b - determinant.sqrt()) / (2 * a)),
            ((-b + determinant.sqrt()) / (2 * a)),
        );
        let keep_in_bounds = |x: i64| {
            if x < 0 {
                0
            } else if (x as u64) > self.time {
                self.time
            } else {
                x as u64
            }
        };
        let (mut lower_bound, mut upper_bound) =
            (keep_in_bounds(x_1 - 1), keep_in_bounds(x_2 + 1));

        // Do some checks to make sure [lower_bound,upper_bound] win
        while !self.check_win(lower_bound) {
            lower_bound += 1;
        }
        while !self.check_win(upper_bound) {
            upper_bound -= 1;
        }

        upper_bound - lower_bound + 1
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
