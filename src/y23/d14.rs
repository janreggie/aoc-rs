use std::{
    fmt::{self, Debug},
    hash::Hash,
};

use anyhow::{Context, Ok, Result};
use bimap::{BiHashMap, BiMap};

use crate::util::vectors::Grid;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Cell {
    RoundedRock,
    CubicRock,
    Space,
}

impl Cell {
    fn new(c: char) -> Option<Cell> {
        match c {
            'O' => Some(Cell::RoundedRock),
            '#' => Some(Cell::CubicRock),
            '.' => Some(Cell::Space),
            _ => None,
        }
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::RoundedRock => 'O',
            Cell::CubicRock => '#',
            Cell::Space => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Platform {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Platform {
    fn new(grid: Grid) -> Option<Platform> {
        let cell_grid = grid
            .chars
            .iter()
            .map(|row| {
                row.iter().map(|c| Cell::new(*c)).collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Platform {
            grid: cell_grid,
            width: grid.width,
            height: grid.height,
        })
    }

    fn at(&self, (pos_x, pos_y): (usize, usize)) -> Cell {
        self.grid[pos_y][pos_x]
    }

    fn set(&mut self, (pos_x, pos_y): (usize, usize), cell: Cell) {
        self.grid[pos_y][pos_x] = cell;
    }

    fn tilt_north(&mut self) {
        for pos_x in 0..self.width {
            // First, grab all cube-shaped positions
            let mut cube_positions = (0..self.height)
                .into_iter()
                .filter(|pos_y| self.at((pos_x, *pos_y)) == Cell::CubicRock)
                .collect::<Vec<_>>();
            cube_positions.push(self.height);
            let starting_cube_position = cube_positions[0];
            let mut ranges = cube_positions
                .windows(2)
                .map(|v1| (v1[0] + 1, v1[1]))
                .collect::<Vec<_>>();
            if starting_cube_position != 0 {
                ranges.insert(0, (0, starting_cube_position));
            }

            // Move all rounded rocks at the beginning
            for (start_pos_y, end_pos_y) in ranges {
                let mut rounded_rock_count = 0;
                for pos_y in start_pos_y..end_pos_y {
                    if self.at((pos_x, pos_y)) == Cell::RoundedRock {
                        rounded_rock_count += 1;
                        self.set((pos_x, pos_y), Cell::Space);
                    }
                }
                for pos_y in start_pos_y..start_pos_y + rounded_rock_count {
                    self.set((pos_x, pos_y), Cell::RoundedRock);
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for pos_y in 0..self.height {
            // First, grab all cube-shaped positions
            let mut cube_positions = (0..self.width)
                .into_iter()
                .filter(|pos_x| self.at((*pos_x, pos_y)) == Cell::CubicRock)
                .collect::<Vec<_>>();
            cube_positions.push(self.width);
            let starting_cube_position = cube_positions[0];
            let mut ranges = cube_positions
                .windows(2)
                .map(|v1| (v1[0] + 1, v1[1]))
                .collect::<Vec<_>>();
            if starting_cube_position != 0 {
                ranges.insert(0, (0, starting_cube_position));
            }

            // Move all rounded rocks at the beginning
            for (start_pos_x, end_pos_x) in ranges {
                let mut rounded_rock_count = 0;
                for pos_x in start_pos_x..end_pos_x {
                    if self.at((pos_x, pos_y)) == Cell::RoundedRock {
                        rounded_rock_count += 1;
                        self.set((pos_x, pos_y), Cell::Space);
                    }
                }
                for pos_x in start_pos_x..start_pos_x + rounded_rock_count {
                    self.set((pos_x, pos_y), Cell::RoundedRock);
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for pos_x in 0..self.width {
            // First, grab all cube-shaped positions
            let mut cube_positions: Vec<usize> = (0..self.height)
                .into_iter()
                .filter(|pos_y| self.at((pos_x, *pos_y)) == Cell::CubicRock)
                .collect::<Vec<_>>();
            cube_positions.push(self.height);

            for ii in 0..cube_positions.len() {
                let end_pos_y = cube_positions[ii];
                let start_pos_y =
                    if ii == 0 { 0 } else { cube_positions[ii - 1] + 1 };
                let mut rounded_rock_count = 0;
                for pos_y in start_pos_y..end_pos_y {
                    if self.at((pos_x, pos_y)) == Cell::RoundedRock {
                        rounded_rock_count += 1;
                        self.set((pos_x, pos_y), Cell::Space);
                    }
                }
                for pos_y in (end_pos_y - rounded_rock_count)..end_pos_y {
                    self.set((pos_x, pos_y), Cell::RoundedRock);
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for pos_y in 0..self.height {
            // First, grab all cube-shaped positions (this is in reverse)
            let mut cube_positions = (0..self.width)
                .into_iter()
                .filter(|pos_x| self.at((*pos_x, pos_y)) == Cell::CubicRock)
                .collect::<Vec<_>>();
            cube_positions.push(self.width);

            for ii in 0..cube_positions.len() {
                let end_pos_x = cube_positions[ii];
                let start_pos_x =
                    if ii == 0 { 0 } else { cube_positions[ii - 1] + 1 };
                let mut rounded_rock_count = 0;
                for pos_x in start_pos_x..end_pos_x {
                    if self.at((pos_x, pos_y)) == Cell::RoundedRock {
                        rounded_rock_count += 1;
                        self.set((pos_x, pos_y), Cell::Space);
                    }
                }
                for pos_x in (end_pos_x - rounded_rock_count)..end_pos_x {
                    self.set((pos_x, pos_y), Cell::RoundedRock);
                }
            }
        }
    }

    fn run_spin_cycle(&mut self, times: usize) {
        // memo contains the state of self before instance ii.
        let mut memo: BiHashMap<Vec<Vec<Cell>>, usize> = BiMap::new();
        for ii in 0..times {
            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            if let Some(&old_ii) = memo.get_by_left(&self.grid.clone()) {
                // This means that, after `old_ii`, `self` is at the same state as `ii`.
                let period = ii - old_ii;
                // Now, we want to know what the state would be at `times-1`.
                // If memo.get_by_right(ii) === memo.get_by_right(ii+period*j) for any integer `j`,
                // then memo.get_by_right(times-1) === memo.get_by_right(times-1-period*j) with sufficiently large j.
                // mm == times-1-period*j, but j should be small enough so that mm is between old_ii and ii.
                let mut mm: usize = (times - 1) % period;
                while mm < old_ii {
                    mm += period;
                }
                let result_grid = memo.get_by_right(&mm).unwrap(); // must exist
                self.grid = result_grid.clone();
                return;
            }

            memo.insert(self.grid.clone(), ii);
        }
    }

    fn calculate_total_load(&self) -> u64 {
        let mut total_load = 0;
        for pos_y in 0..self.height {
            for pos_x in 0..self.width {
                if self.at((pos_x, pos_y)) == Cell::RoundedRock {
                    total_load += self.height - pos_y;
                }
            }
        }
        total_load as u64
    }
}

impl Debug for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pos_y in 0..self.height {
            for pos_x in 0..self.width {
                write!(f, "{:?}", self.at((pos_x, pos_y)))?;
            }
            writeln!(f, "")?;
        }
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn platform_from_str(input: &str) -> Platform {
        let input =
            input.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
        Platform::new(Grid::new(&input).unwrap()).unwrap()
    }

    const EXAMPLE_PLATFORM: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn test_tilt_north() {
        let mut platform = platform_from_str(EXAMPLE_PLATFORM);
        platform.tilt_north();
        assert_eq!(
            platform,
            platform_from_str(
                "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
            )
        );
    }

    #[test]
    fn test_tilt_west() {
        let mut platform = platform_from_str(EXAMPLE_PLATFORM);
        platform.tilt_west();
        assert_eq!(
            platform,
            platform_from_str(
                "O....#....
OOO.#....#
.....##...
OO.#OO....
OO......#.
O.#O...#.#
O....#OO..
O.........
#....###..
#OO..#...."
            )
        );
    }

    #[test]
    fn test_tilt_south() {
        let mut platform = platform_from_str(EXAMPLE_PLATFORM);
        platform.tilt_south();
        assert_eq!(
            platform,
            platform_from_str(
                ".....#....
....#....#
...O.##...
...#......
O.O....O#O
O.#..O.#.#
O....#....
OO....OO..
#OO..###..
#OO.O#...O"
            )
        );
    }

    #[test]
    fn test_tilt_east() {
        let mut platform = platform_from_str(EXAMPLE_PLATFORM);
        platform.tilt_east();
        assert_eq!(
            platform,
            platform_from_str(
                "....O#....
.OOO#....#
.....##...
.OO#....OO
......OO#.
.O#...O#.#
....O#..OO
.........O
#....###..
#..OO#...."
            )
        );
    }

    #[test]
    fn test_count_total_load() {
        let mut platform = platform_from_str(EXAMPLE_PLATFORM);
        platform.tilt_north();
        assert_eq!(platform.calculate_total_load(), 136);
    }
}

fn solve_part_1(platform: &Platform) -> Result<String> {
    let mut platform = platform.clone();
    platform.tilt_north();
    Ok(platform.calculate_total_load().to_string())
}

fn solve_part_2(platform: &Platform) -> Result<String> {
    let mut platform = platform.clone();
    platform.run_spin_cycle(1000000000);
    Ok(platform.calculate_total_load().to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let platform =
        Platform::new(Grid::new(&lines).context("cannot parse as grid")?)
            .context("cannot parse as platform")?;
    Ok((solve_part_1(&platform), solve_part_2(&platform)))
}
