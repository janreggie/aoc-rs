use anyhow::{bail, Ok, Result};

use super::d10::Knot;

fn knot_hash(input: &str) -> [u8; 16] {
    let mut lengths =
        input.as_bytes().iter().map(|x| *x as usize).collect::<Vec<_>>();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut knot = Knot::new();
    for _ in 0..64 {
        for length in &lengths {
            knot.twist(*length);
        }
    }
    knot.dense_hash()
}

/// Disk that's currently being defragemented
struct Disk {
    // TODO: Use bitvec::BitArray, see <https://lib.rs/crates/bitvec>
    grid: [[bool; 128]; 128],
}

impl Disk {
    fn new(key: &str) -> Disk {
        let mut grid = [[false; 128]; 128];
        for row in 0..128 {
            let row_input = format!("{}-{}", key, row);
            let row_hash = knot_hash(&row_input);
            for ii in 0..16 {
                for bb in 0..8 {
                    let is_bit_set = row_hash[ii] & (1 << (7 - bb)) != 0;
                    grid[row][ii * 8 + bb] = is_bit_set;
                }
            }
        }

        Disk { grid }
    }

    fn count_used(&self) -> usize {
        self.grid.map(|row| row.iter().filter(|b| **b).count()).iter().sum()
    }

    fn count_regions(&self) -> usize {
        #[derive(PartialEq)]
        enum State {
            Empty,
            Unvisited,
            Consideration,
            Visited,
        }
        let mut grid = self.grid.map(|row| {
            row.map(|b| if b { State::Unvisited } else { State::Empty })
        });

        fn visit(
            grid: &mut [[State; 128]; 128],
            row: usize,
            col: usize,
        ) -> bool {
            if grid[row][col] != State::Unvisited {
                return false;
            }

            let mut queue = vec![(row, col)];
            while let Some((row, col)) = queue.pop() {
                if row > 0 && grid[row - 1][col] == State::Unvisited {
                    grid[row - 1][col] = State::Consideration;
                    queue.push((row - 1, col))
                }
                if row < 127 && grid[row + 1][col] == State::Unvisited {
                    grid[row + 1][col] = State::Consideration;
                    queue.push((row + 1, col))
                }
                if col > 0 && grid[row][col - 1] == State::Unvisited {
                    grid[row][col - 1] = State::Consideration;
                    queue.push((row, col - 1))
                }
                if col < 127 && grid[row][col + 1] == State::Unvisited {
                    grid[row][col + 1] = State::Consideration;
                    queue.push((row, col + 1))
                }
                grid[row][col] = State::Visited;
            }

            true
        }

        let mut region_count = 0;
        for row in 0..128 {
            for col in 0..128 {
                if visit(&mut grid, row, col) {
                    region_count += 1;
                }
            }
        }

        region_count
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expected only 1 line as input, got {}", lines.len())
    }
    let input = lines.into_iter().next().unwrap();
    let disk = Disk::new(&input);

    // Part 1: Number of squares (ones) in the grid
    let ans1 = Ok(disk.count_used().to_string());

    // Part 2: Island counting
    let ans2 = Ok(disk.count_regions().to_string());

    Ok((ans1, ans2))
}
