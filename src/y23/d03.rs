use std::vec;

use anyhow::{Context, Result};
use itertools::Itertools;

use crate::util::vectors::Grid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SchematicChar {
    // Period
    Empty,
    // Any digit
    Digit(u32),
    // A symbol e.g., `*`, `#`
    Symbol(char),
}

impl SchematicChar {
    fn new(c: char) -> SchematicChar {
        match c {
            '.' => SchematicChar::Empty,
            '0'..='9' => SchematicChar::Digit(c.to_digit(10).unwrap()),
            _ => SchematicChar::Symbol(c),
        }
    }
}

struct Schematic {
    width: usize,
    height: usize,
    chars: Vec<Vec<SchematicChar>>,
}

impl Schematic {
    fn new(grid: &Grid) -> Schematic {
        Schematic {
            width: grid.width,
            height: grid.height,
            chars: grid
                .chars
                .iter()
                .map(|row| row.iter().map(|c| SchematicChar::new(*c)).collect())
                .collect(),
        }
    }

    /// Lists all symbols, each having the format (SchematicChar::Symbol,(pos_x, pos_y))
    fn list_symbols(&self) -> Vec<(SchematicChar, (usize, usize))> {
        self.chars
            .iter()
            .enumerate()
            .map(|(pos_y, list)| {
                list.into_iter()
                    .enumerate()
                    .filter_map(|(pos_x, c)| match c {
                        SchematicChar::Symbol(_) => Some((*c, (pos_x, pos_y))),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
    }

    /// Lists all part numbers and returns entries containing (number, (pos_x_1, pos_x_2), pos_y)
    /// such that the part number is located between grid[pos_y][pos_x_1] and grid[pos_y][pos_x_2-1]
    fn list_part_numbers(&self) -> Vec<(u32, (usize, usize), usize)> {
        self.chars
            .iter()
            .enumerate()
            .map(|(pos_y, row)| {
                Schematic::list_part_numbers_per_row(row)
                    .into_iter()
                    .map(|(number, (pos_x_1, pos_x_2))| {
                        (number, (pos_x_1, pos_x_2), pos_y)
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    /// Lists part numbers in a vector of chars, together with their starting and ending positions.
    /// That is, an entry (num, (pos_x1, pos_x2)) will have a (pos_x2-pos_x1+1)-digit number `num` from `row[pos_x1]` to `pos[pos_x2+1]`.
    fn list_part_numbers_per_row(
        row: &Vec<SchematicChar>,
    ) -> Vec<(u32, (usize, usize))> {
        let mut result = vec![];
        let mut starting_index = None;
        let mut current_number = 0;

        for index in 0..row.len() {
            let c = row[index];
            match c {
                SchematicChar::Digit(d) => {
                    if starting_index.is_none() {
                        starting_index = Some(index);
                        current_number = d;
                    } else {
                        current_number = current_number * 10 + d;
                    }
                }
                _ => {
                    if let Some(index_0) = starting_index {
                        result.push((current_number, (index_0, index - 1)));
                        starting_index = None;
                    }
                }
            }
        }

        if let Some(index_0) = starting_index {
            result.push((current_number, (index_0, row.len() - 1)));
        }

        result
    }

    /// Returns whether there's a symbol *surrounding* the position (pos_x, pos_y).
    fn has_symbol_surrounding(&self, pos_x: usize, pos_y: usize) -> bool {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        offsets
            .into_iter()
            .map(|(o_1, o_2)| (pos_x as i32 + o_1, pos_y as i32 + o_2))
            .filter_map(|(pos_x, pos_y)| {
                if pos_x >= 0
                    && pos_y >= 0
                    && pos_x < self.width as i32
                    && pos_y < self.height as i32
                {
                    Some((pos_x as usize, pos_y as usize))
                } else {
                    None
                }
            })
            .any(|(pos_x, pos_y)| match self.chars[pos_y][pos_x] {
                SchematicChar::Symbol(_) => true,
                _ => false,
            })
    }

    fn find_surrounding_number_pair(
        &self,
        pos_x: usize,
        pos_y: usize,
    ) -> Option<(u32, u32)> {
        let offsets = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        let positions = offsets
            .into_iter()
            .map(|(o_1, o_2)| (pos_x as i32 + o_1, pos_y as i32 + o_2))
            .filter_map(|(pos_x, pos_y)| {
                if pos_x >= 0
                    && pos_y >= 0
                    && pos_x < self.width as i32
                    && pos_y < self.height as i32
                {
                    Some((pos_x as usize, pos_y as usize))
                } else {
                    None
                }
            })
            .filter_map(|(pos_x, pos_y)| match self.chars[pos_y][pos_x] {
                SchematicChar::Digit(_) => Some((pos_x, pos_y)),
                _ => None,
            });
        let positions_by_row = positions
            .group_by(|(_pos_x, pos_y)| *pos_y)
            .into_iter()
            .map(|(pos_y, group)| (pos_y, group.collect()))
            .collect::<Vec<(usize, Vec<_>)>>();

        // Now, do some analysis.
        match positions_by_row.len() {
            0 => None,
            1 => {
                let (_, positions) = &positions_by_row[0];
                // The only way for this to work is if the two digits have a gap in between
                if positions.len() == 2
                    && positions[0].0.abs_diff(positions[1].0) == 2
                {
                    Some((
                        self.get_number(positions[0]),
                        self.get_number(positions[1]),
                    ))
                } else {
                    None
                }
            }
            2 => {
                // We have to make sure that each row has *one and only one* part number.
                // This lambda function assumes that positions contains digits.
                let check_if_row_has_only_one_part_number =
                    |positions: &Vec<(usize, usize)>| match positions.len() {
                        0 => false,
                        1 => true,
                        2 => positions[0].0.abs_diff(positions[1].0) == 1,
                        3 => true,
                        _ => false,
                    };
                let ((_, first_positions), (_, second_positions)) =
                    (&positions_by_row[0], &positions_by_row[1]);
                if check_if_row_has_only_one_part_number(first_positions)
                    && check_if_row_has_only_one_part_number(second_positions)
                {
                    Some((
                        self.get_number(first_positions[0]),
                        self.get_number(second_positions[0]),
                    ))
                } else {
                    None
                }
            }
            _ => None, // invalid i.e., shouldn't happen
        }
    }

    /// Returns the part number at the given position, or 0 if it doesn't exist
    fn get_number(&self, (pos_x, pos_y): (usize, usize)) -> u32 {
        let get_digit = |pos_x: usize| {
            self.chars.get(pos_y).and_then(|row| {
                row.get(pos_x).and_then(|c| match c {
                    SchematicChar::Digit(d) => Some(*d),
                    _ => None,
                })
            })
        };
        let current_digit = get_digit(pos_x);
        if current_digit.is_none() {
            return 0;
        }

        // Numbers can only be at most three digits.
        // The "if pos_x >= 1" checks are so that the numbers don't underflow.
        let mut result = current_digit.unwrap();
        if pos_x >= 1 {
            if let Some(d) = get_digit(pos_x - 1) {
                result = d * 10 + result;
                if pos_x >= 2 {
                    if let Some(d) = get_digit(pos_x - 2) {
                        result = d * 100 + result;
                    }
                }
            }
        }
        if let Some(d) = get_digit(pos_x + 1) {
            result = result * 10 + d;
            if let Some(d) = get_digit(pos_x + 2) {
                result = result * 10 + d;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_numbers_per_row() {
        let schematic = Schematic::new(
            &Grid::new(&vec![
                //123456789
                "467..114..".to_string(),
                "...*......".to_string(),
                "..35..633.".to_string(),
            ])
            .unwrap(),
        );
        let part_numbers = schematic
            .chars
            .iter()
            .map(Schematic::list_part_numbers_per_row)
            .collect::<Vec<_>>();
        assert_eq!(
            part_numbers,
            vec![
                vec![(467, (0, 2)), (114, (5, 7))],
                vec![],
                vec![(35, (2, 3)), (633, (6, 8))]
            ]
        )
    }

    #[test]
    fn test_find_surrounding_number_pair() {
        let schematic = Schematic::new(
            &Grid::new(&vec![
                "467..114..".to_string(),
                "...*......".to_string(),
                "..35..633.".to_string(),
                "......#...".to_string(),
                "617*......".to_string(),
                ".....+.58.".to_string(),
                "..592.....".to_string(),
                "......755.".to_string(),
                "...$.*....".to_string(),
                ".664.598..".to_string(),
            ])
            .unwrap(),
        );
        assert_eq!(schematic.get_number((0, 0)), 467);

        let inputs = vec![(3, 1), (5, 8)];
        let outputs = vec![Some((467, 35)), Some((755, 598))];
        assert_eq!(
            inputs
                .into_iter()
                .map(|(pos_x, pos_y)| schematic
                    .find_surrounding_number_pair(pos_x, pos_y))
                .collect::<Vec<_>>(),
            outputs
        );
    }
}

fn solve_part_1(schematic: &Schematic) -> Result<String> {
    let all_part_numbers = schematic.list_part_numbers();
    let ans1: u32 = all_part_numbers
        .into_iter()
        .filter_map(|(number, (pos_x_1, pos_x_2), pos_y)| {
            // NOTE: We can only do this because we have the assumption that part numbers are at most 3 digits long.
            // In fact, this also supports four digit numbers, but will break when the part numbers have five digits, and the symbols are above or below digit 3.
            if schematic.has_symbol_surrounding(pos_x_1, pos_y)
                || schematic.has_symbol_surrounding(pos_x_2, pos_y)
            {
                Some(number)
            } else {
                None
            }
        })
        .sum();
    Ok(ans1.to_string())
}

fn solve_part_2(schematic: &Schematic) -> Result<String> {
    let all_gear_positions = schematic
        .list_symbols()
        .into_iter()
        .filter_map(|(c, pos)| match c {
            SchematicChar::Symbol(s) => {
                if s == '*' {
                    Some(pos)
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    let all_gear_ratios = all_gear_positions
        .into_iter()
        .filter_map(|(pos_x, pos_y)| {
            schematic
                .find_surrounding_number_pair(pos_x, pos_y)
                .and_then(|pair| Some(((pos_x, pos_y), pair)))
        })
        .collect::<Vec<_>>();
    for (pos, pair) in &all_gear_ratios {
        println!("{:?} has pair {:?}", pos, pair);
    }
    let all_gear_ratios = all_gear_ratios.into_iter().map(|(_, pair)| pair);
    let ans2 = all_gear_ratios.map(|(num_1, num_2)| num_1 * num_2).sum::<u32>();
    Ok(ans2.to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let grid = Grid::new(&lines).context("cannot create grid")?;
    let schematic = Schematic::new(&grid);

    Ok((solve_part_1(&schematic), solve_part_2(&schematic)))
}
