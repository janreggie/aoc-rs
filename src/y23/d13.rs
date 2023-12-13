use std::fmt;

use anyhow::{Context, Ok, Result};

use crate::util::vectors::{group, Grid};

/// Mirror determines where the mirror is in a Pattern.
/// Vertical(pos_x) means that Pattern.at(pos_x,*) == Pattern.at(pos_x+1,*);  Pattern.at(pos_x-1,*) == Pattern.at(pos_x+2,*), etc.
/// Horizontal(pos_y) means that Pattern.at(*,pos_y) == Pattern.at(*,pos_y+1);  Pattern.at(*,pos_y-1) == Pattern.at(*,pos_y+2), etc.
#[derive(Debug, PartialEq, Eq)]
enum Mirror {
    Vertical(usize),
    Horizontal(usize),
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Ash,
    Rock,
}

impl Cell {
    fn new(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Ash),
            '#' => Some(Cell::Rock),
            _ => None,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Ash => '.',
            Cell::Rock => '#',
        };
        write!(f, "{}", c)
    }
}

struct Pattern {
    grid: Vec<Vec<Cell>>,
    height: usize,
    width: usize,
}

impl Pattern {
    fn new(grid: &Grid) -> Option<Pattern> {
        let cell_grid = grid
            .chars
            .iter()
            .map(|row| {
                row.iter().map(|c| Cell::new(*c)).collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Pattern {
            grid: cell_grid,
            height: grid.height,
            width: grid.width,
        })
    }

    fn at(&self, (pos_x, pos_y): (usize, usize)) -> Cell {
        self.grid[pos_y][pos_x]
    }

    fn find_mirror(&self) -> Option<Mirror> {
        for candidate_pos_x in 0..self.width - 1 {
            if self.check_vertical_mirror(candidate_pos_x) {
                return Some(Mirror::Vertical(candidate_pos_x));
            }
        }
        for candidate_pos_y in 0..self.height - 1 {
            if self.check_horizontal_mirror(candidate_pos_y) {
                return Some(Mirror::Horizontal(candidate_pos_y));
            }
        }

        None
    }

    fn find_smudged_mirror(&self) -> Option<Mirror> {
        for candidate_pos_x in 0..self.width - 1 {
            if self.check_smudged_vertical_mirror(candidate_pos_x) {
                return Some(Mirror::Vertical(candidate_pos_x));
            }
        }
        for candidate_pos_y in 0..self.height - 1 {
            if self.check_smudged_horizontal_mirror(candidate_pos_y) {
                return Some(Mirror::Horizontal(candidate_pos_y));
            }
        }

        None
    }

    /// Checks if it's possible that the vertical mirrror is at some position
    fn check_vertical_mirror(&self, candidate_pos_x: usize) -> bool {
        for ii in 0.. {
            if candidate_pos_x < ii || candidate_pos_x + 1 + ii >= self.width {
                break;
            }
            let (left_pos_x, right_pos_x) =
                (candidate_pos_x - ii, candidate_pos_x + 1 + ii);
            for pos_y in 0..self.height {
                if self.at((left_pos_x, pos_y)) != self.at((right_pos_x, pos_y))
                {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if it's possible that the horizontal mirrror is at some position
    fn check_horizontal_mirror(&self, candidate_pos_y: usize) -> bool {
        for ii in 0.. {
            if candidate_pos_y < ii || candidate_pos_y + 1 + ii >= self.height {
                break;
            }
            let (up_pos_y, down_pos_y) =
                (candidate_pos_y - ii, candidate_pos_y + 1 + ii);
            for pos_x in 0..self.width {
                if self.at((pos_x, up_pos_y)) != self.at((pos_x, down_pos_y)) {
                    return false;
                }
            }
        }
        true
    }

    /// Checks if it's possible that the smudged vertical mirrror is at some position
    fn check_smudged_vertical_mirror(&self, candidate_pos_x: usize) -> bool {
        let mut mistakes = 0;
        for ii in 0.. {
            if candidate_pos_x < ii || candidate_pos_x + 1 + ii >= self.width {
                break;
            }
            let (left_pos_x, right_pos_x) =
                (candidate_pos_x - ii, candidate_pos_x + 1 + ii);
            for pos_y in 0..self.height {
                if self.at((left_pos_x, pos_y)) != self.at((right_pos_x, pos_y))
                {
                    mistakes += 1;
                }
                if mistakes >= 2 {
                    return false;
                }
            }
        }
        mistakes == 1
    }

    /// Checks if it's possible that the smudged horizontal mirrror is at some position
    fn check_smudged_horizontal_mirror(&self, candidate_pos_y: usize) -> bool {
        let mut mistakes = 0;
        for ii in 0.. {
            if candidate_pos_y < ii || candidate_pos_y + 1 + ii >= self.height {
                break;
            }
            let (up_pos_y, down_pos_y) =
                (candidate_pos_y - ii, candidate_pos_y + 1 + ii);
            for pos_x in 0..self.width {
                if self.at((pos_x, up_pos_y)) != self.at((pos_x, down_pos_y)) {
                    mistakes += 1;
                }
                if mistakes >= 2 {
                    return false;
                }
            }
        }
        mistakes == 1
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pos_y in 0..self.height {
            for pos_x in 0..self.width {
                write!(f, "{}", self.at((pos_x, pos_y)))?
            }
            writeln!(f, "")?
        }
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pattern_from_str(grid: &str) -> Pattern {
        Pattern::new(
            &Grid::new(
                &grid
                    .split('\n')
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_find_vertical_mirror() {
        let pattern = pattern_from_str(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(pattern.find_mirror(), Some(Mirror::Vertical(4)));
    }

    #[test]
    fn test_find_horizontal_mirror() {
        let pattern = pattern_from_str(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(pattern.find_mirror(), Some(Mirror::Horizontal(3)));
    }

    #[test]
    fn test_find_smudged_mirrors() {
        let p1 = pattern_from_str(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.",
        );
        assert_eq!(p1.find_smudged_mirror(), Some(Mirror::Horizontal(2)));
        let p2 = pattern_from_str(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(p2.find_smudged_mirror(), Some(Mirror::Horizontal(0)));
    }
}

fn solve_part_1(patterns: &Vec<Pattern>) -> Result<String> {
    Ok(patterns
        .iter()
        .map(|pattern| {
            pattern
                .find_mirror()
                .context(format!("cannot find mirror for {}", pattern))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|mirror| match mirror {
            Mirror::Vertical(v) => v + 1,
            Mirror::Horizontal(h) => 100 * (h + 1),
        })
        .sum::<usize>()
        .to_string())
}

fn solve_part_2(patterns: &Vec<Pattern>) -> Result<String> {
    Ok(patterns
        .iter()
        .map(|pattern| {
            pattern
                .find_smudged_mirror()
                .context(format!("cannot find smudged mirror for {}", pattern))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .map(|mirror| match mirror {
            Mirror::Vertical(v) => v + 1,
            Mirror::Horizontal(h) => 100 * (h + 1),
        })
        .sum::<usize>()
        .to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let patterns = group(lines)
        .iter()
        .map(|group| Grid::new(group).and_then(|grid| Pattern::new(&grid)))
        .collect::<Option<Vec<_>>>()
        .context("cannot parse all input")?;
    Ok((solve_part_1(&patterns), solve_part_2(&patterns)))
}
