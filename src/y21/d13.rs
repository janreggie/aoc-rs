use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::cmp::max;
use std::collections::HashSet;

#[derive(Clone, Debug)]
struct Paper {
    points: Vec<(u32, u32)>,
    width: u32,
    height: u32,
}

impl Paper {
    fn new(lines: Vec<String>) -> Result<Paper> {
        let mut points = Vec::new();
        let (mut width, mut height) = (0, 0);

        for line in lines {
            let coords = vectors::split_and_trim(&line, ',');
            if coords.len() != 2 {
                bail!(
                    "expected `{}` to be split into 2, got {}",
                    line,
                    coords.len()
                );
            }
            let coords: Vec<u32> = vectors::from_strs(&coords)
                .context(format!("could not format `{}` into u32", line))?;
            let (x, y) = (coords[0], coords[1]);
            width = max(width, x + 1);
            height = max(height, y + 1);

            points.push((x, y));
        }

        Ok(Paper { points, width, height })
    }

    fn count_points(&self) -> usize {
        self.points.len()
    }

    // Folds the paper by interpreting some instruction and returns the new paper.
    // Returns an error if there is some point in the paper that might overflow.
    fn fold(&self, instr: &FoldInstr) -> Result<Paper> {
        let mut points = Vec::new();
        let mut point_hashes = HashSet::new();
        let (mut width, mut height) = (0, 0);

        for pt in &self.points {
            let (mut x, mut y) = *pt;
            match instr {
                FoldInstr::X(c) => {
                    if x > *c {
                        if x > 2 * *c {
                            bail!("x coordinate overflow {}", x)
                        }
                        x = 2 * *c - x;
                    }
                }
                FoldInstr::Y(c) => {
                    if y > *c {
                        if y > 2 * *c {
                            bail!("y coordinate overflow {}", y)
                        }
                        y = 2 * *c - y;
                    }
                }
            }
            width = max(width, x + 1);
            height = max(height, y + 1);

            let pt = (x, y);
            let hash = Paper::point_to_hash(&pt);
            if point_hashes.contains(&hash) {
                continue;
            }
            points.push(pt);
            point_hashes.insert(hash);
        }

        Ok(Paper { points, width, height })
    }

    fn point_to_hash(input: &(u32, u32)) -> u32 {
        input.0 * 10000 + input.1
    }
}

type Instructions = Vec<FoldInstr>;

fn new_instructions(lines: Vec<String>) -> Result<Instructions> {
    let instructions: Result<Vec<FoldInstr>> = lines
        .into_iter()
        .map(|ip| {
            FoldInstr::new(&ip)
                .context(format!("could not convert {} into FoldInstr", ip))
        })
        .collect();
    let instructions = instructions.context("could not parse instructions")?;
    Ok(instructions)
}

#[derive(Debug)]
enum FoldInstr {
    X(u32),
    Y(u32),
}

impl FoldInstr {
    fn new(input: &str) -> Result<FoldInstr> {
        let mut input = vectors::split_and_trim(input, ' ');
        if input.len() != 3 {
            bail!("could not interpret FoldInstr");
        }
        let input = input.pop().unwrap();

        let input = vectors::split_and_trim(&input, '=');
        if input.len() != 2 {
            bail!("could not interpret last element as FoldInstr");
        }

        let axis = &input[0];
        let count: u32 = input[1]
            .parse()
            .context(format!("could not interpret count {}", &input[1]))?;

        match axis.as_str() {
            "x" => Ok(FoldInstr::X(count)),
            "y" => Ok(FoldInstr::Y(count)),
            _ => bail!("could not interpret axis {}", axis),
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let mut groups = vectors::group(lines);
    if groups.len() != 2 {
        bail!("expected input to be split into 2, got {}", groups.len());
    }
    let instructions = groups.pop().unwrap();
    let instructions = new_instructions(instructions)
        .context("could not create Instructions")?;
    if instructions.len() < 1 {
        bail!("zero instructions")
    }
    let paper =
        Paper::new(groups.pop().unwrap()).context("could not create Paper")?;

    // Part 1: Parse the first instruction, and only the first instruction
    let first_instruction = &instructions[0];
    let folded_paper =
        paper.fold(first_instruction).context("could not fold paper")?;
    let ans1 = Ok(folded_paper.count_points().to_string());

    // Part 2: Fold according to all instructions
    let mut paper = paper;
    for instr in &instructions {
        paper = paper
            .fold(instr)
            .context(format!("could not fold using instruction {:?}", instr))?;
    }
    // Let's find a way to display this to user output
    let mut ans2: Vec<Vec<bool>> =
        vec![vec![false; paper.width as usize]; paper.height as usize];
    for point in &paper.points {
        let (x, y) = *point;
        ans2[y as usize][x as usize] = true;
    }
    let ans2: Vec<String> = ans2
        .iter()
        .map(|row| row.iter().map(|b| if *b { '█' } else { ' ' }).collect())
        .collect();
    let ans2 = ans2.join("\n");
    let ans2 = Ok(String::from("the following:\n") + &ans2);

    Ok((ans1, ans2))
}
