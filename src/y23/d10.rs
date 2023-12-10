use std::{collections::HashSet, fmt};

use anyhow::{bail, Context, Result};

use crate::util::vectors::Grid;

#[derive(Clone, Copy)]
enum Tile {
    Pipe(Direction, Direction),
    Ground,
    Starting,
}

impl Tile {
    fn new(c: char) -> Option<Tile> {
        match c {
            '|' => Some(Tile::Pipe(Direction::North, Direction::South)),
            '-' => Some(Tile::Pipe(Direction::West, Direction::East)),
            'L' => Some(Tile::Pipe(Direction::North, Direction::East)),
            'J' => Some(Tile::Pipe(Direction::North, Direction::West)),
            '7' => Some(Tile::Pipe(Direction::South, Direction::West)),
            'F' => Some(Tile::Pipe(Direction::South, Direction::East)),
            '.' => Some(Tile::Ground),
            'S' => Some(Tile::Starting),
            _ => None,
        }
    }

    /// Returns whether Tile is a Pipe and has some Direction
    fn has_direction(&self, direction: Direction) -> bool {
        match self {
            Tile::Pipe(d1, d2) => *d1 == direction || *d2 == direction,
            _ => false,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Tile::Pipe(d11, d12), Tile::Pipe(d21, d22)) => {
                (d11 == d21 && d12 == d22) || (d11 == d22 && d12 == d21)
            }
            (Tile::Ground, Tile::Ground) => true,
            (Tile::Starting, Tile::Starting) => true,
            _ => false,
        }
    }
}

impl Eq for Tile {}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Tile::Pipe(Direction::North, Direction::South) => '│',
            Tile::Pipe(Direction::West, Direction::East) => '─',
            Tile::Pipe(Direction::North, Direction::East) => '└',
            Tile::Pipe(Direction::North, Direction::West) => '┘',
            Tile::Pipe(Direction::South, Direction::West) => '┐',
            Tile::Pipe(Direction::South, Direction::East) => '┌',
            Tile::Pipe(_, _) => '?',
            Tile::Ground => '·',
            Tile::Starting => 'S',
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn rev(&self) -> Direction {
        match self {
            Direction::North => Self::South,
            Direction::South => Self::North,
            Direction::West => Self::East,
            Direction::East => Self::West,
        }
    }
}

struct Maze {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Eq, Debug)]
enum MazeRowState {
    Outside,
    Inside,
    ToInsideFrom(Direction),
    ToOutsideFrom(Direction),
}

impl Maze {
    fn new(grid: &Grid) -> Option<Maze> {
        let tiles = grid
            .chars
            .iter()
            .map(|line| {
                line.iter().map(|c| Tile::new(*c)).collect::<Option<Vec<_>>>()
            })
            .collect::<Option<Vec<Vec<_>>>>()?;
        Some(Maze { tiles, width: grid.width, height: grid.height })
    }

    /// at((pos_x,pos_y)) is shorthand for tiles[pos_y][pos_x].
    fn at(&self, (pos_x, pos_y): (usize, usize)) -> Tile {
        self.tiles[pos_y][pos_x]
    }

    /// Returns the farthest distance from starting position.
    fn get_farthest_distance(&self) -> Result<usize> {
        let mut position = self
            .get_starting_position()
            .context("cannot find starting position")?;
        let mut direction = Direction::East; // doesn't matter; next_move will get a valid direction for us
        let mut count_steps = 0;

        loop {
            (direction, position) =
                self.next_move(direction, position).context(format!(
                    "cannot figure out next move at {:?} from {:?}",
                    position, direction
                ))?;
            count_steps += 1;
            if self.at(position) == Tile::Starting {
                break;
            }
        }
        Ok(count_steps / 2)
    }

    /// Infer what type of tile Starting is
    fn infer_starting_tile(&self) -> Result<Tile> {
        let starting_position = self.get_starting_position()?;
        let directions = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .iter()
        // Don't include the same direction we came from
        // Then make sure that we're even able to go to said direction.
        .filter_map(|d| {
            self._check_next_pos(*d, starting_position)
                .and_then(|pos| Some((*d, pos)))
        })
        // Direction is reversed here for the pipe check i.e., "if we're going North,
        // we wanna make sure the pipe up north has a South direction"
        .filter(|&(d, starting_position)| {
            self.at(starting_position).has_direction(d.rev())
        })
        .map(|(d, _)| d)
        .collect::<Vec<Direction>>();
        if directions.len() != 2 {
            bail!("invalid directions for starting {:?}", directions)
        }
        Ok(Tile::Pipe(directions[0], directions[1]))
    }

    /// Counts how many tiles there are inside the loop
    fn count_enclosed_tiles(&self) -> Result<usize> {
        let mut position = self
            .get_starting_position()
            .context("cannot find starting position")?;
        let mut direction = Direction::East; // doesn't matter; next_move will get a valid direction for us

        let mut border_positions = HashSet::new();
        border_positions.insert(position);

        loop {
            let (next_direction, next_position) =
                self.next_move(direction, position).context(format!(
                    "cannot figure out next move at {:?} from {:?}",
                    position, direction
                ))?;
            if self.at(next_position) == Tile::Starting {
                break;
            }
            (position, direction) = (next_position, next_direction);
            border_positions.insert(position);
        }

        let mut count_tiles = 0;
        for pos_y in 0..self.height {
            // Every row begins in being "outside" the border
            let mut current_row_state = MazeRowState::Outside;
            for pos_x in 0..self.width {
                if border_positions.contains(&(pos_x, pos_y)) {
                    current_row_state = self
                        .next_row_state(
                            current_row_state,
                            self.at((pos_x, pos_y)),
                        )
                        .context(format!(
                            "cannot get next row state at {} {}",
                            pos_x, pos_y
                        ))?;
                }
                if current_row_state == MazeRowState::Inside
                    && !border_positions.contains(&(pos_x, pos_y))
                {
                    count_tiles += 1;
                }
            }
        }
        Ok(count_tiles)
    }

    /// next_move(West, (pos_x, pos_y)) evaluates what the next move should be when entering (pos_x,pos_y) from the West.
    /// If pos_x,pos_y is a Pipe(North, West), then it should return (South, (pos_x, pos_y+1)).
    fn next_move(
        &self,
        direction: Direction,
        (pos_x, pos_y): (usize, usize),
    ) -> Result<(Direction, (usize, usize))> {
        let current = self.at((pos_x, pos_y));
        match current {
            Tile::Pipe(d1, d2) => {
                let next_direction = if d1 == direction.rev() {
                    d2
                } else if d2 == direction.rev() {
                    d1
                } else {
                    bail!(
                        "cannot enter cell {} at ({},{}) from {:?}",
                        current,
                        pos_x,
                        pos_y,
                        direction
                    );
                };
                Ok((
                    next_direction,
                    self._check_next_pos(next_direction, (pos_x, pos_y))
                        .context("overflow")?,
                ))
            }
            Tile::Ground => bail!("ground at ({},{})", pos_x, pos_y),
            Tile::Starting => {
                let inferred_starting_tile = self
                    .infer_starting_tile()
                    .context("cannot infer starting tile directions")?;
                let next_direction =
                    if let Tile::Pipe(d1, d2) = inferred_starting_tile {
                        if d1 == direction {
                            d2
                        } else {
                            d1
                        }
                    } else {
                        bail!("inferred_starting_tile must be Tile::Pipe")
                    };
                Ok((
                    next_direction,
                    self._check_next_pos(next_direction, (pos_x, pos_y))
                        .context(
                            "cannot get next position from starting tile",
                        )?,
                ))
            }
        }
    }

    /// Returns (pos_x, pos_y) such that self.tiles[pos_y][pos_x] == Tile::Starting
    fn get_starting_position(&self) -> Result<(usize, usize)> {
        for pos_x in 0..self.width {
            for pos_y in 0..self.height {
                if self.tiles[pos_y][pos_x] == Tile::Starting {
                    return Ok((pos_x, pos_y));
                }
            }
        }
        bail!("cannot find starting tile")
    }

    /// Evaluates if it's possible to go to Direction from (pos_x,pos_y) and returns the new position if so
    fn _check_next_pos(
        &self,
        direction: Direction,
        (pos_x, pos_y): (usize, usize),
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::North => {
                if pos_y >= 1 {
                    return Some((pos_x, pos_y - 1));
                }
            }
            Direction::South => {
                if pos_y + 1 < self.height {
                    return Some((pos_x, pos_y + 1));
                }
            }
            Direction::West => {
                if pos_x >= 1 {
                    return Some((pos_x - 1, pos_y));
                }
            }
            Direction::East => {
                if pos_x + 1 < self.width {
                    return Some((pos_x + 1, pos_y));
                }
            }
        }
        None
    }

    /// Gets the next row previous_row_state, given the previous one, and the current position
    fn next_row_state(
        &self,
        previous_row_state: MazeRowState,
        current_tile: Tile,
    ) -> Result<MazeRowState> {
        // The situation may only flip when there's a single vertical pipe │
        // or a continuous series e.g., └┐, └──┐.
        match (&previous_row_state, current_tile) {
            (MazeRowState::Outside, Tile::Pipe(_, _)) => {
                let has_north = current_tile.has_direction(Direction::North);
                let has_south = current_tile.has_direction(Direction::South);
                match (has_north, has_south) {
                    (true, true) => Ok(MazeRowState::Inside),
                    (true, false) => {
                        Ok(MazeRowState::ToInsideFrom(Direction::North))
                    }
                    (false, true) => {
                        Ok(MazeRowState::ToInsideFrom(Direction::South))
                    }
                    (false, false) => bail!(
                        "cannot enter from outside to tile {}",
                        current_tile,
                    ),
                }
            }
            (MazeRowState::Inside, Tile::Pipe(_, _)) => {
                let has_north = current_tile.has_direction(Direction::North);
                let has_south = current_tile.has_direction(Direction::South);
                match (has_north, has_south) {
                    (true, true) => Ok(MazeRowState::Outside),
                    (true, false) => {
                        Ok(MazeRowState::ToOutsideFrom(Direction::North))
                    }
                    (false, true) => {
                        Ok(MazeRowState::ToOutsideFrom(Direction::South))
                    }
                    (false, false) => bail!(
                        "cannot enter from inside to tile {}",
                        current_tile,
                    ),
                }
            }
            (MazeRowState::ToInsideFrom(d), Tile::Pipe(_, _)) => {
                let has_north = current_tile.has_direction(Direction::North);
                let has_south = current_tile.has_direction(Direction::South);
                if has_north && *d == Direction::South
                    || has_south && *d == Direction::North
                {
                    Ok(MazeRowState::Inside)
                } else if has_north && *d == Direction::North
                    || has_south && *d == Direction::South
                {
                    Ok(MazeRowState::Outside)
                } else {
                    Ok(previous_row_state)
                }
            }
            (MazeRowState::ToOutsideFrom(d), Tile::Pipe(_, _)) => {
                let has_north = current_tile.has_direction(Direction::North);
                let has_south = current_tile.has_direction(Direction::South);
                if has_north && *d == Direction::South
                    || has_south && *d == Direction::North
                {
                    Ok(MazeRowState::Outside)
                } else if has_north && *d == Direction::North
                    || has_south && *d == Direction::South
                {
                    Ok(MazeRowState::Inside)
                } else {
                    Ok(previous_row_state)
                }
            }
            (_, Tile::Ground) => Ok(previous_row_state),
            (_, Tile::Starting) => self.next_row_state(
                previous_row_state,
                self.infer_starting_tile()
                    .context("cannot get starting tile")?,
            ),
        }
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            write!(f, "\n")?;
        }
        return fmt::Result::Ok(());
    }
}

fn solve_part_1(maze: &Maze) -> Result<String> {
    Ok(maze
        .get_farthest_distance()
        .context("cannot get farthest distance")?
        .to_string())
}

fn solve_part_2(maze: &Maze) -> Result<String> {
    Ok(maze
        .count_enclosed_tiles()
        .context("cannot count enclosed ground tiles")?
        .to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let grid = Grid::new(&lines).context("cannot create grid")?;
    let maze = Maze::new(&grid).context("cannot parse grid to cells")?;

    Ok((solve_part_1(&maze), solve_part_2(&maze)))
}
