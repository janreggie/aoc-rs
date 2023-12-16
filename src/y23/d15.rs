use std::{collections::VecDeque, fmt};

use anyhow::{bail, Context, Ok, Result};

/// Calculates the hash of input string. Returns a value from 0 to 255.
fn hash(input: &str) -> u32 {
    let mut result = 0;
    for c in input.chars() {
        result += c as u32;
        result *= 17;
        result = result & 0xff; // mod 256
    }
    result
}

#[derive(PartialEq, Eq)]
enum Operation {
    Dash,
    Equals(usize),
}

impl fmt::Debug for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Dash => write!(f, "-"),
            Operation::Equals(fl) => write!(f, "={}", fl),
        }
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Eq)]
struct Step {
    label: String,
    operation: Operation,
}

impl Step {
    fn new(input: &str) -> Option<Step> {
        if let Some((label, _)) = input.split_once('-') {
            Some(Step { label: label.to_string(), operation: Operation::Dash })
        } else if let Some((label, focal_length)) = input.split_once('=') {
            let focal_length = focal_length.parse::<usize>().ok()?;
            if focal_length < 1 || focal_length > 9 {
                return None;
            }
            Some(Step {
                label: label.to_string(),
                operation: Operation::Equals(focal_length),
            })
        } else {
            None
        }
    }
}

impl fmt::Debug for Step {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.label, self.operation)
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn from(step: Step) -> Option<Lens> {
        let focal_length = if let Operation::Equals(fl) = step.operation {
            fl
        } else {
            return None;
        };
        Some(Lens { label: step.label, focal_length })
    }
}

struct Boxes {
    boxes: Vec<VecDeque<Lens>>,
}

impl Boxes {
    fn new() -> Boxes {
        Boxes { boxes: vec![VecDeque::new(); 256] }
    }

    fn insert(&mut self, step: Step) {
        let hash = hash(&step.label) as usize;
        let current_box = &mut self.boxes[hash];
        match &step.operation {
            Operation::Dash => {
                // Look for the lens containing a certain label
                let mut label = None;
                for ii in 0..current_box.len() {
                    if current_box[ii].label == step.label {
                        label = Some(ii);
                        break;
                    }
                }
                if let Some(ii) = label {
                    current_box.remove(ii);
                }
            }
            Operation::Equals(_) => {
                let new_lens = Lens::from(step).unwrap();
                for ii in 0..current_box.len() {
                    if current_box[ii].label == new_lens.label {
                        current_box[ii] = new_lens;
                        return;
                    }
                }
                current_box.push_back(new_lens);
            }
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut result = 0;
        for box_ii in 0..self.boxes.len() {
            let current_box = &self.boxes[box_ii];
            for slot_ii in 0..current_box.len() {
                result += ((box_ii + 1)
                    * (slot_ii + 1)
                    * current_box[slot_ii].focal_length)
                    as u64
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let cases = vec![
            ("rn=1", 30),
            ("cm-", 253),
            ("qp=3", 97),
            ("cm=2", 47),
            ("qp-", 14),
            ("pc=4", 180),
            ("ot=9", 9),
            ("ab=5", 197),
            ("pc-", 48),
            ("pc=6", 214),
            ("ot=7", 231),
        ];
        for (input, expected) in cases {
            assert_eq!(hash(input), expected);
        }
    }

    #[test]
    fn test_step_new() {
        let cases = vec![
            (
                "rn=1",
                Step {
                    label: "rn".to_string(),
                    operation: Operation::Equals(1),
                },
            ),
            (
                "pc-",
                Step { label: "pc".to_string(), operation: Operation::Dash },
            ),
        ];
        for (input, expected) in cases {
            assert_eq!(Step::new(input), Some(expected))
        }
    }
}

fn solve_part_1(input: &Vec<String>) -> Result<String> {
    Ok(input.iter().map(|s| hash(s)).sum::<u32>().to_string())
}

fn solve_part_2(input: &Vec<String>) -> Result<String> {
    let mut boxes = Boxes::new();
    let steps = input
        .iter()
        .map(|input| {
            Step::new(&input)
                .context(format!("cannot parse {:?} into Step", input))
        })
        .collect::<Result<Vec<_>>>()
        .context("cannot parse all of input")?;
    for step in steps {
        boxes.insert(step);
    }
    Ok(boxes.focusing_power().to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expect one line, got {}", lines.len());
    }
    let input =
        lines[0].split(',').map(|s| s.to_string()).collect::<Vec<String>>();

    Ok((solve_part_1(&input), solve_part_2(&input)))
}
