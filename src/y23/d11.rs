use std::{collections::HashSet, fmt::Display};

use anyhow::{Context, Result};

use crate::util::vectors::Grid;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Elem {
    Space,
    Galaxy,
}

impl Elem {
    fn new(c: char) -> Option<Elem> {
        match c {
            '.' => Some(Elem::Space),
            '#' => Some(Elem::Galaxy),
            _ => None,
        }
    }
}

impl Display for Elem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Elem::Space => '.',
            Elem::Galaxy => '#',
        };
        write!(f, "{}", c)
    }
}

struct Image {
    grid: Vec<Vec<Elem>>,
    width: usize,
    height: usize,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Image {
    fn new(grid: Grid) -> Result<Image> {
        let elem_grid = grid
            .chars
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|c| Elem::new(c))
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()
            .context("cannot parse grid: invalid characters")?;
        let mut empty_rows = HashSet::new();
        'outer: for pos_y in 0..grid.height {
            for pos_x in 0..grid.width {
                if elem_grid[pos_y][pos_x] == Elem::Galaxy {
                    continue 'outer;
                }
            }
            empty_rows.insert(pos_y);
        }
        let mut empty_cols = HashSet::new();
        'outer: for pos_x in 0..grid.width {
            for pos_y in 0..grid.height {
                if elem_grid[pos_y][pos_x] == Elem::Galaxy {
                    continue 'outer;
                }
            }
            empty_cols.insert(pos_x);
        }

        Ok(Image {
            grid: elem_grid,
            width: grid.width,
            height: grid.height,
            empty_rows,
            empty_cols,
        })
    }

    fn at(&self, (pos_x, pos_y): (usize, usize)) -> Elem {
        self.grid[pos_y][pos_x]
    }

    /// Gets the sum of shortest paths, where empty_distance is the size of the empty rows.
    /// That is, in Part 1, empty_distance was 2, while in Part 2 it would be 1_000_000.
    fn get_sum_of_shortest_paths(&self, empty_distance: usize) -> usize {
        // Grab all positions of galaxies
        let mut positions = vec![];
        for pos_x in 0..self.width {
            for pos_y in 0..self.height {
                if self.at((pos_x, pos_y)) == Elem::Galaxy {
                    positions.push((pos_x, pos_y));
                }
            }
        }

        // Then, grab the list of rows and columns which contain galaxies
        // (and how many galaxies there are in that row/col)
        let (valid_cols, valid_rows): (Vec<_>, Vec<_>) =
            positions.into_iter().unzip();
        let valid_rows = Self::remove_dupes_and_count(valid_rows);
        let valid_cols = Self::remove_dupes_and_count(valid_cols);

        // Now, permute
        let mut total_sum = 0;
        for ii in 0..valid_rows.len() {
            for jj in ii + 1..valid_rows.len() {
                let (first_row, first_row_count) = valid_rows[ii];
                let (second_row, second_row_count) = valid_rows[jj];
                let current_distance = second_row - first_row
                    + (self.count_empty_rows_between(first_row, second_row)
                        * (empty_distance - 1));
                total_sum +=
                    current_distance * first_row_count * second_row_count;
            }
        }
        for ii in 0..valid_cols.len() {
            for jj in ii + 1..valid_cols.len() {
                let (first_col, first_col_count) = valid_cols[ii];
                let (second_col, second_col_count) = valid_cols[jj];
                let current_distance = second_col - first_col
                    + (self.count_empty_columns_between(first_col, second_col)
                        * (empty_distance - 1));
                total_sum +=
                    current_distance * first_col_count * second_col_count;
            }
        }

        total_sum
    }

    fn remove_dupes_and_count(mut vals: Vec<usize>) -> Vec<(usize, usize)> {
        vals.sort();
        let mut result = vec![];
        for elem in vals {
            if result.is_empty() {
                result.push((elem, 1));
            } else {
                let last = result.last_mut().unwrap();
                if last.0 == elem {
                    last.1 += 1;
                } else {
                    result.push((elem, 1));
                }
            }
        }
        result
    }

    /// Counts how many empty rows there are between two y-pos's
    fn count_empty_rows_between(&self, row_1: usize, row_2: usize) -> usize {
        let mut count = 0;
        for ii in row_1 + 1..row_2 {
            if self.empty_rows.contains(&ii) {
                count += 1;
            }
        }
        count
    }

    /// Counts how many empty columns there are between two y-pos's
    fn count_empty_columns_between(&self, col_1: usize, col_2: usize) -> usize {
        let mut count = 0;
        for ii in col_1 + 1..col_2 {
            if self.empty_cols.contains(&ii) {
                count += 1;
            }
        }
        count
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for col in row {
                write!(f, "{}", col)?
            }
            writeln!(f)?
        }
        std::fmt::Result::Ok(())
    }
}

fn solve_part_1(image: &Image) -> Result<String> {
    Ok(image.get_sum_of_shortest_paths(2).to_string())
}

fn solve_part_2(image: &Image) -> Result<String> {
    Ok(image.get_sum_of_shortest_paths(1_000_000).to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let grid = Grid::new(&lines).context("cannot create grid")?;
    let image = Image::new(grid).context("cannot create image")?;

    Ok((solve_part_1(&image), solve_part_2(&image)))
}
