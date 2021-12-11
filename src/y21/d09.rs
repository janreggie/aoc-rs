use anyhow::{bail, Context, Result};

#[derive(Debug)]
struct Heightmap {
    heights: Vec<Vec<u32>>,
    x_len: usize,
    y_len: usize,
}

impl Heightmap {
    fn new(lines: Vec<String>) -> Result<Heightmap> {
        if lines.len() == 0 || lines.first().unwrap().len() == 0 {
            bail!("cannot parse empty lines")
        }

        let mut heights = Vec::new();
        let x_len = lines.first().unwrap().len();
        let y_len = lines.len();

        for line in lines {
            if line.len() != x_len {
                bail!(
                    "expected line `{}` to be of length {}, got {}",
                    line,
                    x_len,
                    line.len()
                )
            }
            let mut heightmap_line = Vec::new();
            for ch in line.chars() {
                match ch.to_digit(10) {
                    None => bail!("could not parse character `{}` from line `{}`", ch, line),
                    Some(d) => heightmap_line.push(d),
                }
            }
            heights.push(heightmap_line);
        }

        Ok(Heightmap {
            heights,
            x_len,
            y_len,
        })
    }

    /// Returns the values surrounding point (x,y) in the order [up,down,left,right].
    /// If it cannot be found (e.g., `up` at the topmost row), then it will be set to `def`.
    fn get_surrounding(&self, x: usize, y: usize, def: u32) -> [u32; 4] {
        let mut result = [def; 4];
        if y > 0 {
            result[0] = self.heights[y - 1][x];
        }
        if y < self.y_len - 1 {
            result[1] = self.heights[y + 1][x];
        }
        if x > 0 {
            result[2] = self.heights[y][x - 1];
        }
        if x < self.x_len - 1 {
            result[3] = self.heights[y][x + 1];
        }
        result
    }

    fn low_points_risk(&self) -> u32 {
        let mut result = 0;
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                let cur = self.heights[y][x];
                let surrounding = self.get_surrounding(x, y, 10);
                if cur < *surrounding.iter().min().unwrap() {
                    result += 1 + cur;
                }
            }
        }

        result
    }

    /// Gets the sizes of the basins, in no particular order
    fn basins(&self) -> Vec<usize> {
        #[derive(Clone, PartialEq, Eq)]
        enum State {
            Forbidden,
            Unvisited,
            Visiting,
            Visited,
        }
        let mut state = vec![vec![State::Unvisited; self.x_len]; self.y_len];
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if self.heights[y][x] == 9 {
                    state[y][x] = State::Forbidden;
                }
            }
        }

        // Finally, let's visit the basins...
        let mut basin_sizes = Vec::new();
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if state[y][x] != State::Unvisited {
                    continue;
                }

                // Do something of a DFS from that node
                let mut size = 0;
                let mut stack = vec![(x, y)];
                while let Some((x, y)) = stack.pop() {
                    state[y][x] = State::Visited;
                    size += 1;

                    // Add the neighbors...
                    if y > 0 && state[y - 1][x] == State::Unvisited {
                        stack.push((x, y - 1));
                        state[y - 1][x] = State::Visiting;
                    }
                    if y < self.y_len - 1 && state[y + 1][x] == State::Unvisited {
                        stack.push((x, y + 1));
                        state[y + 1][x] = State::Visiting;
                    }
                    if x > 0 && state[y][x - 1] == State::Unvisited {
                        stack.push((x - 1, y));
                        state[y][x - 1] = State::Visiting;
                    }
                    if x < self.x_len - 1 && state[y][x + 1] == State::Unvisited {
                        stack.push((x + 1, y));
                        state[y][x + 1] = State::Visiting;
                    }
                }
                basin_sizes.push(size)
            }
        }

        basin_sizes
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let heightmap = Heightmap::new(lines).context("could not generate heightmap")?;

    // Part 1: Risk of low points
    let ans1 = heightmap.low_points_risk();

    // Part 2: Sizes of three largest basins
    let mut basin_sizes = heightmap.basins();
    basin_sizes.sort();
    basin_sizes.reverse();
    let ans2: usize = basin_sizes.iter().take(3).product();

    Ok((ans1.to_string(), ans2.to_string()))
}
