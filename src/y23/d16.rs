use anyhow::{Context, Ok, Result};
use std::{
    collections::{HashSet, VecDeque},
    fmt,
};

use crate::util::vectors::Grid;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Empty,              // .
    MirrorForward,      // /
    MirrorBackward,     // \
    SplitterVertical,   // |
    SplitterHorizontal, // -
}

impl Cell {
    fn new(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Empty),
            '/' => Some(Cell::MirrorForward),
            '\\' => Some(Cell::MirrorBackward),
            '|' => Some(Cell::SplitterVertical),
            '-' => Some(Cell::SplitterHorizontal),
            _ => None,
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Empty => '.',
            Cell::MirrorForward => '/',
            Cell::MirrorBackward => '\\',
            Cell::SplitterVertical => '|',
            Cell::SplitterHorizontal => '-',
        };
        write!(f, "{}", c)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}
#[derive(PartialEq, Eq, Clone, Hash)]
struct Step {
    pos: (usize, usize),
    direction: Direction,
}

struct Layout {
    grid: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Layout {
    fn new(grid: Grid) -> Option<Layout> {
        let cell_grid = grid
            .chars
            .iter()
            .map(|row| {
                row.iter().map(|c| Cell::new(*c)).collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Layout { grid: cell_grid, height: grid.height, width: grid.width })
    }

    fn at(&self, (pos_x, pos_y): (usize, usize)) -> Cell {
        self.grid[pos_y][pos_x]
    }

    fn count_energized_from(&self, first_step: Step) -> usize {
        let mut energized_cells = vec![vec![false; self.width]; self.height];
        let mut steps = VecDeque::new();
        steps.push_back(first_step.clone());
        let mut steps_to_be_done = HashSet::new();
        steps_to_be_done.insert(first_step);
        let mut completed_steps = HashSet::new();

        // Calculates what the next step should be after moving to step.direction
        let next_step = |step: &Step| -> Option<Step> {
            match step.direction {
                Direction::North => {
                    if step.pos.1 == 0 {
                        None
                    } else {
                        Some(Step {
                            pos: (step.pos.0, step.pos.1 - 1),
                            direction: step.direction,
                        })
                    }
                }
                Direction::West => {
                    if step.pos.0 == 0 {
                        None
                    } else {
                        Some(Step {
                            pos: (step.pos.0 - 1, step.pos.1),
                            direction: step.direction,
                        })
                    }
                }
                Direction::South => {
                    if step.pos.1 + 1 >= self.height {
                        None
                    } else {
                        Some(Step {
                            pos: (step.pos.0, step.pos.1 + 1),
                            direction: step.direction,
                        })
                    }
                }
                Direction::East => {
                    if step.pos.0 + 1 >= self.width {
                        None
                    } else {
                        Some(Step {
                            pos: (step.pos.0 + 1, step.pos.1),
                            direction: step.direction,
                        })
                    }
                }
            }
        };

        loop {
            if steps.len() == 0 {
                break;
            }
            let step = steps.pop_front().unwrap();
            energized_cells[step.pos.1][step.pos.0] = true;
            steps_to_be_done.remove(&step);
            completed_steps.insert(step.clone());

            let mut next_steps = vec![];
            match self.at(step.pos) {
                Cell::Empty => {
                    next_steps.push(next_step(&step));
                }
                Cell::MirrorForward => {
                    let next_direction = match step.direction {
                        Direction::North => Direction::East,
                        Direction::West => Direction::South,
                        Direction::South => Direction::West,
                        Direction::East => Direction::North,
                    };
                    let mut step = step;
                    step.direction = next_direction;
                    next_steps.push(next_step(&step));
                }
                Cell::MirrorBackward => {
                    let next_direction = match step.direction {
                        Direction::North => Direction::West,
                        Direction::West => Direction::North,
                        Direction::South => Direction::East,
                        Direction::East => Direction::South,
                    };
                    let mut step = step;
                    step.direction = next_direction;
                    next_steps.push(next_step(&step));
                }
                Cell::SplitterVertical => {
                    if step.direction == Direction::North
                        || step.direction == Direction::South
                    {
                        next_steps.push(next_step(&step));
                    } else {
                        let (mut north_step, mut south_step) =
                            (step.clone(), step);
                        north_step.direction = Direction::North;
                        south_step.direction = Direction::South;
                        next_steps.push(next_step(&north_step));
                        next_steps.push(next_step(&south_step));
                    }
                }
                Cell::SplitterHorizontal => {
                    if step.direction == Direction::West
                        || step.direction == Direction::East
                    {
                        next_steps.push(next_step(&step));
                    } else {
                        let (mut west_step, mut east_step) =
                            (step.clone(), step);
                        west_step.direction = Direction::West;
                        east_step.direction = Direction::East;
                        next_steps.push(next_step(&west_step));
                        next_steps.push(next_step(&east_step));
                    }
                }
            }
            for next_step in next_steps {
                if let Some(next_step) = next_step {
                    if !steps_to_be_done.contains(&next_step)
                        && !completed_steps.contains(&next_step)
                    {
                        steps.push_back(next_step.clone());
                        steps_to_be_done.insert(next_step);
                    }
                }
            }
        }

        energized_cells
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

fn solve_part_1(layout: &Layout) -> Result<String> {
    let first_step = Step { pos: (0, 0), direction: Direction::East };
    Ok(layout.count_energized_from(first_step).to_string())
}

fn solve_part_2(layout: &Layout) -> Result<String> {
    let mut first_steps = vec![];
    for pos_x in 0..layout.width {
        first_steps.push(Step { pos: (pos_x, 0), direction: Direction::North });
        first_steps.push(Step { pos: (pos_x, 0), direction: Direction::South });
    }
    for pos_y in 0..layout.height {
        first_steps.push(Step { pos: (0, pos_y), direction: Direction::West });
        first_steps.push(Step { pos: (0, pos_y), direction: Direction::East });
    }

    let mut record = 0;
    for first_step in first_steps {
        let current = layout.count_energized_from(first_step);
        record = record.max(current);
    }
    Ok(record.to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let layout = Layout::new(Grid::new(&lines).context("cannot create grid")?)
        .context("cannot create layout")?;

    Ok((solve_part_1(&layout), solve_part_2(&layout)))
}
