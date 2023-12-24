use anyhow::{bail, Context, Ok, Result};
use sscanf::{sscanf, FromScanf};
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use crate::util::vectors::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    /// Reachable in `.0` steps from center. Reachable(0) means the center point
    Reachable(u32),
    /// Unreachable because it's a rock.
    Rock,
    /// Unreachable from the Starting position because possibly surrounded by rocks.
    Unreachable,
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(grid: Grid) -> Result<Map> {
        let mut cells = grid
            .chars
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| match c {
                        'S' => Some(Cell::Reachable(0)),
                        '#' => Some(Cell::Rock),
                        '.' => Some(Cell::Unreachable),
                        _ => None,
                    })
                    .collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()
            .context("cannot parse all characters")?;

        let (width, height) = (grid.width, grid.height);
        let next_positions = |(pos_x, pos_y): (usize, usize)| {
            let mut result = vec![];
            if pos_x >= 1 {
                result.push((pos_x - 1, pos_y));
            }
            if pos_x + 1 < width {
                result.push((pos_x + 1, pos_y));
            }
            if pos_y >= 1 {
                result.push((pos_x, pos_y - 1));
            }
            if pos_y + 1 < height {
                result.push((pos_x, pos_y + 1));
            }
            result
        };

        // Look for the starting position
        let mut starting_position = None;
        'l: for pos_x in 0..width {
            for pos_y in 0..height {
                if cells[pos_y][pos_x] == Cell::Reachable(0) {
                    starting_position = Some((pos_x, pos_y));
                    break 'l;
                }
            }
        }
        if starting_position == None {
            bail!("cannot find starting position")
        }
        let starting_position = starting_position.unwrap();

        // Then look for Reachables
        let mut queue = VecDeque::new();
        queue.push_back((starting_position, 0));
        while let Some(((pos_x, pos_y), cur_count)) = queue.pop_front() {
            if cur_count != 0 && cells[pos_y][pos_x] != Cell::Unreachable {
                continue;
            }
            cells[pos_y][pos_x] = Cell::Reachable(cur_count);
            for (next_pos_x, next_pos_y) in next_positions((pos_x, pos_y)) {
                if cells[next_pos_y][next_pos_x] != Cell::Unreachable {
                    continue;
                }
                queue.push_back(((next_pos_x, next_pos_y), cur_count + 1));
            }
        }

        Ok(Map { cells, width, height })
    }

    fn at(&self, (pos_x, pos_y): (usize, usize)) -> Cell {
        self.cells[pos_y][pos_x]
    }

    /// Counts how many steps are reachable given the parameter
    fn count_reachable_at(&self, steps: u32) -> u32 {
        // The space must match the parity of `steps`; i.e., if steps is odd, then the space must be odd.
        // In addition, it should be at most `steps`.
        let mut result = 0;
        for pos_x in 0..self.width {
            for pos_y in 0..self.height {
                match self.at((pos_x, pos_y)) {
                    Cell::Reachable(c) => {
                        if c <= steps && (c + steps) % 2 == 0 {
                            result += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_MAP: &str = "
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    const EXAMPLE_MAP_EXPANDED: &str =
        ".......................................................
.....###.#......###.#......###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#........#.#........#.#....
.##...####..##...####..##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#..##..#...#..##..#...#.
.......##.........##.........##.........##.........##..
.##.#.####..##.#.####..##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##..##..##.##..##..##.##.
.......................................................
.......................................................
.....###.#......###.#......###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#........#.#........#.#....
.##...####..##...####..##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#..##..#...#..##..#...#.
.......##.........##.........##.........##.........##..
.##.#.####..##.#.####..##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##..##..##.##..##..##.##.
.......................................................
.......................................................
.....###.#......###.#......###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#........#.#........#.#....
.##...####..##...####..##..S####..##...####..##...####.
.##..#...#..##..#...#..##..#...#..##..#...#..##..#...#.
.......##.........##.........##.........##.........##..
.##.#.####..##.#.####..##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##..##..##.##..##..##.##.
.......................................................
.......................................................
.....###.#......###.#......###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#........#.#........#.#....
.##...####..##...####..##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#..##..#...#..##..#...#.
.......##.........##.........##.........##.........##..
.##.#.####..##.#.####..##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##..##..##.##..##..##.##.
.......................................................
.......................................................
.....###.#......###.#......###.#......###.#......###.#.
.###.##..#..###.##..#..###.##..#..###.##..#..###.##..#.
..#.#...#....#.#...#....#.#...#....#.#...#....#.#...#..
....#.#........#.#........#.#........#.#........#.#....
.##...####..##...####..##...####..##...####..##...####.
.##..#...#..##..#...#..##..#...#..##..#...#..##..#...#.
.......##.........##.........##.........##.........##..
.##.#.####..##.#.####..##.#.####..##.#.####..##.#.####.
.##..##.##..##..##.##..##..##.##..##..##.##..##..##.##.
.......................................................";
    fn example_map() -> Map {
        Map::new(
            Grid::new(
                &EXAMPLE_MAP
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn test_count_reachable_at() {
        let map = example_map();
        assert_eq!(map.count_reachable_at(1), 2);
        assert_eq!(map.count_reachable_at(2), 4);
        assert_eq!(map.count_reachable_at(3), 6);
        assert_eq!(map.count_reachable_at(6), 16);
    }
}

fn solve_part_1(map: &Map) -> Result<String> {
    Ok(map.count_reachable_at(64).to_string())
}

fn solve_part_2(map: &Map) -> Result<String> {
    for pos_y in 0..map.height {
        for pos_x in 0..map.width {
            let s = match map.at((pos_x, pos_y)) {
                Cell::Reachable(v) => format!("{:03}", v),
                Cell::Rock => "###".to_string(),
                Cell::Unreachable => "!!!".to_string(),
            };
            print!("{} ", s);
        }
        println!()
    }
    bail!("bruh")
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let grid = Grid::new(&lines).context("cannot create grid from input")?;
    let map = Map::new(grid).context("cannot create map from grid")?;
    Ok((solve_part_1(&map), solve_part_2(&map)))
}
