use std::{fmt, str::FromStr};

use anyhow::{self, bail, Context, Result};
use itertools::Itertools;
use sscanf::sscanf;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => bail!("cannot parse direction"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Step {
    direction: Direction,
    step_count: isize,

    // Derived from the "color"
    real_direction: Direction,
    real_step_count: isize,
}

impl FromStr for Step {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, step_count, color) =
            sscanf!(s, "{} {} (#{})", String, isize, String)
                .ok()
                .context("cannot parse Step")?;
        let direction = Direction::from_str(&direction)
            .context(format!("cannot parse direction {:?}", direction))?;

        if color.len() != 6 {
            bail!("cannot parse color {}: should be of length 6", color);
        }
        let (real_step_count, real_direction) = color.split_at(5);
        let real_step_count = isize::from_str_radix(real_step_count, 16)
            .context(format!("cannot parse step count from color {}", color))?;
        let real_direction = match real_direction {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => bail!("cannot parse direction from color {}", color),
        };

        Ok(Step { direction, step_count, real_direction, real_step_count })
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn add(&self, step: &Step, is_part_two: bool) -> Point {
        let direction =
            if is_part_two { step.real_direction } else { step.direction };
        let step_count =
            if is_part_two { step.real_step_count } else { step.step_count };

        let mut new_x = self.x;
        let mut new_y = self.y;
        match direction {
            Direction::Left => new_x -= step_count,
            Direction::Right => new_x += step_count,
            Direction::Up => new_y += step_count,
            Direction::Down => new_y -= step_count,
        }

        Point { x: new_x, y: new_y }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

fn get_points_from_steps(steps: &Vec<Step>, is_part_two: bool) -> Vec<Point> {
    let mut points = vec![Point { x: 0, y: 0 }];
    let mut current_point = Point { x: 0, y: 0 };
    for step in steps {
        current_point = current_point.add(step, is_part_two);
        points.push(current_point);
    }
    points
}

fn get_area(points: &Vec<Point>) -> u64 {
    // This, unfortunately, isn't as simple as just applying the shoelace formula.
    // There's *something* missing---but what?
    todo!()
}

fn solve_part_1(steps: &Vec<Step>) -> Result<String> {
    let points = get_points_from_steps(steps, false);
    Ok(get_area(&points).to_string())
}

fn solve_part_2(steps: &Vec<Step>) -> Result<String> {
    let points = get_points_from_steps(steps, true);
    Ok(get_area(&points).to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let steps = lines
        .iter()
        .map(|line| {
            Step::from_str(line)
                .context(format!("cannot parse line `{}`", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("cannot parse all of input")?;

    Ok((solve_part_1(&steps), solve_part_2(&steps)))
}
