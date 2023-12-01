use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::fmt;

struct Image {
    enhancement_algorithm: [bool; 512],
    image: Vec<Vec<bool>>,
    space: bool,
    width: usize,
    height: usize,
}

impl Image {
    fn new(lines: Vec<String>) -> Result<Image> {
        let groups = vectors::group(lines);
        if groups.len() != 2 {
            bail!(
                "expected to split lines into 2, got {} instead",
                groups.len()
            )
        }
        let algorithm_input = &groups[0];
        if algorithm_input.len() != 1 {
            bail!(
                "expected first group to be of one line, got {} instead",
                algorithm_input.len()
            )
        }
        let algorithm_input = &algorithm_input[0];
        if algorithm_input.len() != 512 {
            bail!(
                "expected enhancement algorithm to be of length 512, got {} instead",
                algorithm_input.len()
            )
        }
        let algorithm_input: Vec<char> = algorithm_input.chars().collect();
        let mut enhancement_algorithm = [false; 512];
        for ii in 0..512 {
            match algorithm_input[ii] {
                '#' => enhancement_algorithm[ii] = true,
                '.' => enhancement_algorithm[ii] = false,
                c => bail!("unknown character {}", c),
            }
        }

        let image_rows = &groups[1];
        if image_rows.len() == 0 || image_rows[0].len() == 0 {
            bail!("empty image rows");
        }
        let (width, height) = (image_rows[0].len(), image_rows.len());
        let mut image = Vec::new();
        for row in image_rows {
            if row.len() != width {
                bail!(
                    "expected row to be of length {}, got {} instead",
                    width,
                    row.len()
                );
            }
            let row: Result<Vec<bool>> = row
                .chars()
                .map(|c| match c {
                    '#' => Ok(true),
                    '.' => Ok(false),
                    c => bail!("unknown character {}", c),
                })
                .collect();
            image.push(row.context("could not parse some character")?);
        }

        Ok(Image { enhancement_algorithm, image, space: false, width, height })
    }

    /// Determine what the value at some pixel (x,y) will be for the *enhanced* image.
    /// That is, img.it(0,0) will refer to the new top-left corner.
    fn at_enhanced(&self, x: usize, y: usize) -> bool {
        // For the current image with current image coordinates,
        // returns either self.image[x-dx][y-dy] if possible, or whatever the background is.
        let g = |dx: usize, dy: usize| -> bool {
            if x < dx || y < dy {
                return self.space;
            }
            let (x, y) = (x - dx, y - dy);
            if x >= self.width || y >= self.height {
                return self.space;
            }

            self.image[y][x]
        };

        self.get_next([
            g(2, 2),
            g(1, 2),
            g(0, 2),
            g(2, 1),
            g(1, 1),
            g(0, 1),
            g(2, 0),
            g(1, 0),
            g(0, 0),
        ])
    }

    /// Iterate the current one
    fn enhance(&mut self) {
        let width = self.width + 2;
        let height = self.height + 2;
        let mut image = vec![vec![false; width]; height];
        for y in 0..height {
            for x in 0..width {
                image[y][x] = self.at_enhanced(x, y);
            }
        }

        if self.enhancement_algorithm[0] && !self.enhancement_algorithm[511] {
            // The input "flashes" bet. all lit and all dark.
            // Thanks, Eric Wastl.
            self.space = !self.space;
        };
        self.width = width;
        self.height = height;
        self.image = image;
    }

    /// Get the next pixel
    fn get_next(&self, pixels: [bool; 9]) -> bool {
        let mut ind = 0;
        for b in pixels {
            ind *= 2;
            if b {
                ind += 1;
            }
        }
        self.enhancement_algorithm[ind]
    }

    fn count_lit(&self) -> usize {
        self.image.iter().map(|row| row.iter().filter(|b| **b).count()).sum()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.image[y][x] { '#' } else { '.' })?
            }
            writeln!(f, "")?
        }
        write!(f, "")
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let mut image = Image::new(lines).context("could not create image")?;

    // Part 1: First two times
    for _ in 0..2 {
        image.enhance();
    }
    let ans1 = Ok(image.count_lit().to_string());

    // Part 2: Okay, let's do it 48 more times
    for _ in 0..48 {
        image.enhance();
    }
    let ans2 = Ok(image.count_lit().to_string());

    Ok((ans1, ans2))
}
