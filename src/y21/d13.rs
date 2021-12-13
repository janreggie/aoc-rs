use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Paper {
    points: Vec<(usize, usize)>,
    point_hashes: HashSet<usize>,
}

impl Paper {
    fn new(lines: Vec<String>) -> Result<Paper> {
        let mut points = Vec::new();
        for line in lines {
            let coords = vectors::split_and_trim(&line, ',');
            if coords.len() != 2 {
                bail!(
                    "expected `{}` to be split into 2, got {}",
                    line,
                    coords.len()
                );
            }
            let coords: Vec<usize> = vectors::from_strs(&coords)
                .context(format!("could not format `{}` into usize", line))?;
            points.push((coords[0], coords[1]));
        }

        let point_hashes = HashSet::from_iter(points.iter().map(Paper::point_to_hash));

        Ok(Paper {
            points,
            point_hashes,
        })
    }

    fn hash_to_point(input: usize) -> (usize, usize) {
        (input / 10000, input % 10000)
    }

    fn point_to_hash(input: &(usize, usize)) -> usize {
        input.0 * 10000 + input.1
    }
}

#[derive(Debug)]
struct Instructions {
    instructions: VecDeque<FoldInstr>,
}

impl Instructions {
    fn new(lines: Vec<String>) -> Result<Instructions> {
        let instructions: Result<VecDeque<FoldInstr>> = lines
            .into_iter()
            .map(|ip| {
                FoldInstr::new(&ip).context(format!("could not convert {} into FoldInstr", ip))
            })
            .collect();
        let instructions = instructions.context("could not parse instructions")?;
        Ok(Instructions { instructions })
    }
}

#[derive(Debug)]
enum FoldInstr {
    X(usize),
    Y(usize),
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
        let count: usize = input[1]
            .parse()
            .context(format!("could not interpret count {}", &input[1]))?;

        match axis.as_str() {
            "x" => Ok(FoldInstr::X(count)),
            "y" => Ok(FoldInstr::Y(count)),
            _ => bail!("could not interpret axis {}", axis),
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut groups = vectors::group(lines);
    if groups.len() != 2 {
        bail!("expected input to be split into 2, got {}", groups.len());
    }
    let instructions =
        Instructions::new(groups.pop().unwrap()).context("could not create Instructions")?;
    let paper = Paper::new(groups.pop().unwrap()).context("could not create Paper")?;

    dbg!(&paper);
    dbg!(&instructions);

    bail!("bruh")
}
