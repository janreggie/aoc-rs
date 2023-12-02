use anyhow::{bail, Context, Result};
use sscanf::sscanf;

#[derive(Debug)]
struct Step {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
    turn_on: bool, // If false, turn off
}

impl Step {
    fn new(input: &str) -> Result<Step> {
        let parsed_input = sscanf!(
            input,
            "{} x={}..{},y={}..{},z={}..{}",
            String,
            i32,
            i32,
            i32,
            i32,
            i32,
            i32
        );
        if let Err(_) = parsed_input {
            bail!("cannot parse input");
        }
        let (instr, x1, x2, y1, y2, z1, z2) = parsed_input.unwrap();

        if x1 > x2 || y1 > y2 || z1 > z2 {
            bail!("ranges must be valid");
        }
        let turn_on;
        match instr.as_str() {
            "on" => turn_on = true,
            "off" => turn_on = false,
            vv => bail!("invalid instruction `{}`", vv),
        }

        Ok(Step { x: (x1, x2), y: (y1, y2), z: (z1, z2), turn_on })
    }
}

struct NaiveCube {
    /// If radius is 50, the Cube is defined
    /// such that x,y,z can be from -50 to 50 inclusive.
    radius: usize,

    points: Vec<Vec<Vec<bool>>>,
}

impl NaiveCube {
    fn new(radius: usize) -> NaiveCube {
        let side_len = radius * 2 + 1;
        let points = vec![vec![vec![false; side_len]; side_len]; side_len];
        NaiveCube { radius, points }
    }

    fn run(&mut self, step: &Step) {
        for x in step.x.0..step.x.1 + 1 {
            for y in step.y.0..step.y.1 + 1 {
                for z in step.z.0..step.z.1 + 1 {
                    self.set(x, y, z, step.turn_on);
                }
            }
        }
    }

    fn set(&mut self, x: i32, y: i32, z: i32, b: bool) {
        if !self.is_in_bounds_pt(x, y, z) {
            return;
        }

        // Now, do the needful
        let r = self.radius as i32;
        let (x, y, z) = (x + r, y + r, z + r);
        self.points[z as usize][y as usize][x as usize] = b;
    }

    fn is_in_bounds_pt(&self, x: i32, y: i32, z: i32) -> bool {
        let radius = self.radius as i32;
        x >= -radius
            && x <= radius
            && y >= -radius
            && y <= radius
            && z >= -radius
            && z <= radius
    }

    fn is_in_bounds_step(&self, step: &Step) -> bool {
        let radius = self.radius as i32;
        let ((x1, x2), (y1, y2), (z1, z2)) = (step.x, step.y, step.z);
        x1 >= -radius
            && x2 <= radius
            && y1 >= -radius
            && y2 <= radius
            && z1 >= -radius
            && z2 <= radius
    }

    fn count_on(&self) -> usize {
        let mut result = 0;
        for sheet in &self.points {
            for row in sheet {
                for pt in row {
                    if *pt {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}

fn solve_part_2() -> Result<String> {
    bail!("unimplemented")
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let steps: Result<Vec<Step>> = lines
        .iter()
        .map(|l| Step::new(l).context(format!("could not parse line `{}`", l)))
        .collect();
    let steps = steps.context("could not parse input")?;

    // Part 1: Cube is represented by a 3D matrix of booleans.
    let mut cube = NaiveCube::new(50);
    for step in &steps {
        if !cube.is_in_bounds_step(step) {
            break;
        }
        cube.run(step);
    }
    let ans1 = Ok(cube.count_on().to_string());

    // Part 2. In my 16GB RAM environment this crashes.
    // TODO: Implement using intervals and whatever
    let ans2 = solve_part_2();

    Ok((ans1, ans2))
}
