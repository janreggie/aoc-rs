use anyhow::{bail, Context, Result};
use itertools::Itertools;

struct Trees {
    heights: Vec<Vec<u8>>,
    row_count: usize,
    col_count: usize,
}

impl Trees {
    fn parse(lines: Vec<String>) -> Result<Trees> {
        if lines.len() == 0 {
            bail!("empty input");
        }

        let heights = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '0'..='9' => Ok(c as u8 - b'0'),
                        _ => bail!("invalid character {}", c),
                    })
                    .collect::<Result<Vec<_>>>()
                    .with_context(|| format!("invalid line {}", line))
            })
            .collect::<Result<Vec<_>>>()
            .with_context(|| format!("could not convert input into heights"))?;

        let row_count = heights.len();
        let col_count_unique = heights
            .iter()
            .map(|line| line.len())
            .unique()
            .collect::<Vec<usize>>();
        if col_count_unique.len() != 1 {
            bail!(
                "could not determine number of columns from column counts {:?}",
                col_count_unique
            );
        }
        let col_count = col_count_unique[0];
        if col_count == 0 {
            bail!("empty input")
        }

        Ok(Trees {
            heights,
            row_count,
            col_count,
        })
    }

    /// Checks visibility using the VISIBLE_FROM_* constants
    fn visibilities(&self) -> Vec<Vec<u8>> {
        let mut result = vec![vec![0; self.col_count]; self.row_count];

        // Check from left
        for rr in 0..self.row_count {
            let mut tallest = 0; // should be -1 instead but -1 would underflow
            for cc in 0..self.col_count {
                if cc == 0 || self.heights[rr][cc] > tallest {
                    result[rr][cc] |= VISIBLE_FROM_LEFT;
                    tallest = tallest.max(self.heights[rr][cc]);
                }
            }
        }

        // Check from above
        for cc in 0..self.col_count {
            let mut tallest = 0;
            for rr in 0..self.row_count {
                if rr == 0 || self.heights[rr][cc] > tallest {
                    result[rr][cc] |= VISIBLE_FROM_ABOVE;
                    tallest = tallest.max(self.heights[rr][cc])
                }
            }
        }

        // Check from right
        for rr in 0..self.row_count {
            let mut tallest = 0;
            result[rr][self.col_count - 1] |= VISIBLE_FROM_RIGHT;
            for cc in (0..self.col_count).rev() {
                if cc == self.col_count - 1 || self.heights[rr][cc] > tallest {
                    result[rr][cc] |= VISIBLE_FROM_RIGHT;
                    tallest = tallest.max(self.heights[rr][cc])
                }
            }
        }

        // Check from below
        for cc in 0..self.col_count {
            let mut tallest = 0;
            for rr in (0..self.row_count).rev() {
                if rr == self.row_count - 1 || self.heights[rr][cc] > tallest {
                    result[rr][cc] |= VISIBLE_FROM_BELOW;
                    tallest = tallest.max(self.heights[rr][cc])
                }
            }
        }

        result
    }

    fn scenic_scores(&self) -> Vec<Vec<usize>> {
        let mut scores = vec![vec![0; self.col_count]; self.row_count];
        for rr in 0..self.row_count {
            for cc in 0..self.col_count {
                scores[rr][cc] = self.scenic_score(rr, cc);
            }
        }
        scores
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        // Skip computation if we're at the edge
        if row == 0 || col == 0 || row == self.row_count - 1 || col == self.col_count - 1 {
            return 0;
        }

        let current_height = self.heights[row][col];
        let mut result = 1;

        // Look left
        let mut horizon = 0;
        for cc in (0..col).rev() {
            horizon += 1;
            if self.heights[row][cc] >= current_height {
                break;
            }
        }
        result *= horizon;

        // Look up
        let mut horizon = 0;
        for rr in (0..row).rev() {
            horizon += 1;
            if self.heights[rr][col] >= current_height {
                break;
            }
        }
        result *= horizon;

        // Look right
        let mut horizon = 0;
        for cc in (col + 1)..self.col_count {
            horizon += 1;
            if self.heights[row][cc] >= current_height {
                break;
            }
        }
        result *= horizon;

        // Look down
        let mut horizon = 0;
        for rr in (row + 1)..self.row_count {
            horizon += 1;
            if self.heights[rr][col] >= current_height {
                break;
            }
        }
        result *= horizon;

        result
    }
}

const VISIBLE_FROM_LEFT: u8 = 1 << 0;
const VISIBLE_FROM_ABOVE: u8 = 1 << 1;
const VISIBLE_FROM_RIGHT: u8 = 1 << 2;
const VISIBLE_FROM_BELOW: u8 = 1 << 3;

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let trees = Trees::parse(lines).context("could not parse input")?;

    // Part 1: Check how many trees are visible
    let ans1 = trees
        .visibilities()
        .iter()
        .map(|row| row.iter().map(|&c| (c > 0) as u32).sum::<u32>())
        .sum::<u32>()
        .to_string();

    // Part 2: Check for highest scenic score
    // It's okay to unwrap here since we're guaranteed that trees is non-empty.
    // let scenic_scores = trees.scenic_scores();
    // for row in &scenic_scores {
    //     for col in row {
    //         print!("{:03} ", col)
    //     }
    //     println!()
    // }
    let ans2 = trees
        .scenic_scores()
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap()
        .to_string();

    Ok((ans1, ans2))
}
