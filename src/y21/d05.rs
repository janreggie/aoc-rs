use anyhow::{bail, Context, Result};
use num;
use num::Integer;
use std::fmt;

const BOUND: usize = 1000;

#[derive(Clone, Debug, PartialEq)]
struct Point(usize, usize);

impl Point {
    /// new("816,14") -> Point(816,14)
    fn new(input: &str) -> Result<Point> {
        let split: Vec<&str> = input.split(',').collect();
        if split.len() != 2 {
            bail!("could not split by commas");
        }

        let c1 = split[0]
            .parse()
            .context(format!("could not parse {} as usize", split[0]))?;
        if c1 >= BOUND {
            bail!("x-coord should be less than {}, got {}", BOUND, c1);
        }

        let c2 = split[1]
            .parse()
            .context(format!("could not parse {} as usize", split[1]))?;
        if c2 >= BOUND {
            bail!("y-coord should be less than {}, got {}", BOUND, c2);
        }

        Ok(Point(c1, c2))
    }

    /// (x,y).add(c,d) = (x+c,y+d)
    fn add(&self, x_step: isize, y_step: isize) -> Point {
        Point(
            (self.0 as isize + x_step) as usize,
            (self.1 as isize + y_step) as usize,
        )
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Line(Point, Point);

impl Line {
    /// new("816,14 -> 748,14") -> Point from (816,14) to (748,14)
    fn new(input: &str) -> Result<Line> {
        let points: Vec<&str> = input.split(" -> ").collect();
        if points.len() != 2 {
            bail!("could not split points");
        }

        let p1 = Point::new(points[0]).context("could not parse first point")?;
        let p2 = Point::new(points[1]).context("could not parse seconod popint")?;
        Ok(Line(p1, p2))
    }

    /// checks if the Line is parallel to the x- or y-axis.
    fn is_axial(&self) -> bool {
        self.0 .0 == self.1 .0 || self.0 .1 == self.1 .1
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Grid(Vec<Vec<usize>>);

impl Grid {
    fn new() -> Grid {
        Grid(vec![vec![0; BOUND]; BOUND])
    }

    fn clear(&mut self) {
        for ii in 0..BOUND {
            for jj in 0..BOUND {
                self.0[ii][jj] = 0;
            }
        }
    }

    fn draw(&mut self, line: &Line) {
        let p1 = &line.0;
        let p2 = &line.1;

        // The input's non-axial lines *should* be 45-deg ones.
        // However, let's just prepare for the worst.
        // This code will draw the Line onto the Grid
        // only on points with whole numbers.
        // Or whatever, it's difficult to explain via text.
        let x_diff: isize = p2.0 as isize - p1.0 as isize;
        let y_diff: isize = p2.1 as isize - p1.1 as isize;
        let gcd: isize = x_diff.abs().gcd(&y_diff.abs());
        let x_step = x_diff / gcd;
        let y_step = y_diff / gcd;

        let mut pt = p1.clone();
        while &pt != p2 {
            self.0[pt.1][pt.0] += 1;
            pt = pt.add(x_step, y_step);
        }
        self.0[pt.1][pt.0] += 1;
    }

    fn count_intersections(&self) -> usize {
        let mut result = 0;
        for ii in 0..BOUND {
            for jj in 0..BOUND {
                if self.0[ii][jj] > 1 {
                    result += 1;
                }
            }
        }
        result
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for ii in 0..BOUND {
            for jj in 0..BOUND {
                let to_write = match self.0[ii][jj] {
                    0 => String::from("."),
                    1..=9 => self.0[ii][jj].to_string(),
                    _ => String::from("X"),
                };
                write!(f, "{}", to_write)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut ls = Vec::new();
    for line in lines {
        let ll = Line::new(&line).context(format!("could not read line `{}`", line))?;
        ls.push(ll);
    }
    let lines = ls;
    let mut grid = Grid::new();

    // Part 1: Draw all points parallel to x- or y- axis.
    for line in &lines {
        if line.is_axial() {
            grid.draw(line);
        }
    }
    let ans1 = grid.count_intersections();

    // Part 2: All the points
    grid.clear();
    for line in &lines {
        grid.draw(line);
    }
    let ans2 = grid.count_intersections();

    Ok((ans1.to_string(), ans2.to_string()))
}
