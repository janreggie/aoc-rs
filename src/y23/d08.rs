use std::collections::HashMap;

use anyhow::{bail, Context, Result};
use num::Integer;
use sscanf::sscanf;

use crate::util::vectors::group;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(c: char) -> Option<Direction> {
        match c {
            'L' => Some(Direction::Left),
            'R' => Some(Direction::Right),
            _ => None,
        }
    }
}

type Directions = Vec<Direction>;

type Element = [char; 3];

fn new_element(input: &str) -> Option<Element> {
    if input.len() != 3 {
        None
    } else {
        let mut input = input.chars();
        Some([
            input.nth(0).unwrap(),
            input.nth(0).unwrap(),
            input.nth(0).unwrap(),
        ])
    }
}

struct Network {
    nodes: HashMap<Element, (Element, Element)>,
}

impl Network {
    fn new(input: &Vec<String>) -> Result<Network> {
        let nodes = input
            .iter()
            .map(|input| {
                Self::new_from_line(input)
                    .context(format!("cannot parse `{}`", input))
            })
            .collect::<Result<HashMap<Element, (Element, Element)>>>()
            .context("cannot parse input")?;

        Ok(Network { nodes })
    }

    /// Takes an element "AAA = (BBB, CCC)" and turns it to (AAA, (BBB, CCC))
    fn new_from_line(input: &str) -> Result<(Element, (Element, Element))> {
        let (source, left, right) = sscanf!(
            input,
            "{:/[0-9A-Z]{3}/} = ({:/[0-9A-Z]{3}/}, {:/[0-9A-Z]{3}/})",
            &str,
            &str,
            &str
        )
        .ok()
        .context("cannot parse input")?;

        let source = new_element(source).context("cannot parse source")?;
        let left = new_element(left).context("cannot parse left")?;
        let right = new_element(right).context("cannot parse right")?;

        Ok((source, (left, right)))
    }

    /// Gets the element at some direction
    fn get(&self, source: Element, direction: Direction) -> Option<Element> {
        let dest = self.nodes.get(&source)?;
        match direction {
            Direction::Left => Some(dest.0),
            Direction::Right => Some(dest.1),
        }
    }
}

fn solve_part_1(directions: &Directions, network: &Network) -> Result<String> {
    let mut step_count = 0;
    let mut source: Element = ['A', 'A', 'A'];
    'l: loop {
        for direction in directions {
            source = network.get(source, *direction).context(format!(
                "cannot get with source {:?} and direction {:?}",
                source, direction,
            ))?;
            step_count += 1;
            if source == ['Z', 'Z', 'Z'] {
                break 'l;
            }
        }
    }
    Ok(step_count.to_string())
}

fn solve_part_2(directions: &Directions, network: &Network) -> Result<String> {
    let sources = network
        .nodes
        .keys()
        .filter(|element| element[2] == 'A')
        .map(|element| *element)
        .collect::<Vec<Element>>();

    // Get the LCM of the time it takes for each source to complete a route.
    let mut step_count_per_source = vec![0 as u64; sources.len()];
    for ii in 0..sources.len() {
        let mut source: Element = sources[ii];
        'l: loop {
            for direction in directions {
                source = network.get(source, *direction).context(format!(
                    "cannot get with source {:?} and direction {:?}",
                    source, direction,
                ))?;
                step_count_per_source[ii] += 1;
                if source[2] == 'Z' {
                    break 'l;
                }
            }
        }
    }

    Ok(step_count_per_source
        .into_iter()
        .reduce(|v1, v2| v1.lcm(&v2))
        .context("empty step counts")?
        .to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let groups = group(lines);
    if groups.len() != 2 {
        bail!("input should be in 2 groups, got {}", groups.len())
    }
    let (directions, network) = (&groups[0], &groups[1]);
    if directions.len() != 1 {
        bail!("expect directions to be only 1 line, got {}", directions.len());
    }
    let directions = directions[0]
        .chars()
        .map(|c| Direction::new(c))
        .collect::<Option<Vec<_>>>()
        .context("cannot parse directions")?;
    let network = Network::new(network).context("cannot parse network")?;

    Ok((
        solve_part_1(&directions, &network),
        solve_part_2(&directions, &network),
    ))
}
