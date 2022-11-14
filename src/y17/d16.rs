use std::collections::HashSet;

use anyhow::{bail, Context, Result};
use bimap::BiHashMap;
use sscanf::scanf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Program(char);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(u8);

#[derive(Debug)]
struct Promenade {
    programs: BiHashMap<Program, Position>,
    // program_to_position: [Program; 5],
    // position_to_program: [Position; 5],
}

impl Promenade {
    fn new() -> Promenade {
        let mut programs = BiHashMap::new();
        for ii in 0u8..5 {
            programs.insert(Program((0x61 + ii) as char), Position(ii));
        }
        Promenade { programs }
    }

    fn perform(&mut self, dance: &DanceMove) -> Result<()> {
        match dance {
            &DanceMove::Spin(s) => self.spin(s),
            &DanceMove::Exchange(pos1, pos2) => self.exchange(pos1, pos2),
            &DanceMove::Partner(prog1, prog2) => self.partner(prog1, prog2),
        }
        .context("could not perform")
    }

    /// Splits self.programs into two, with programs whose positions are less than position, and those no less than position.
    fn split(&self, position: u8) -> (Vec<(Program, Position)>, Vec<(Program, Position)>) {
        let (lhs, rhs): (Vec<_>, Vec<_>) = self
            .programs
            .iter()
            .partition(|(_prog, &pos)| pos < Position(position));
        let lhs = lhs
            .iter()
            .map(|(prog, pos)| (**prog, **pos))
            .collect::<Vec<_>>();
        let rhs = rhs
            .iter()
            .map(|(prog, pos)| (**prog, **pos))
            .collect::<Vec<_>>();
        (lhs, rhs)
    }

    fn spin(&mut self, spin_size: u8) -> Result<()> {
        let (to_right, to_left): (Vec<_>, Vec<_>) = self.split(5 - spin_size);
        for (prog, pos) in to_left {
            self.programs.insert(prog, Position(pos.0 + spin_size - 5));
        }
        for (prog, pos) in to_right {
            self.programs.insert(prog, Position(pos.0 + spin_size));
        }
        Ok(())
    }

    fn exchange(&mut self, pos1: Position, pos2: Position) -> Result<()> {
        let prog1 = self.get_program(pos1)?;
        let prog2 = self.get_program(pos2)?;
        self.programs.insert(prog1, pos2);
        self.programs.insert(prog2, pos1);
        Ok(())
    }

    fn partner(&mut self, prog1: Program, prog2: Program) -> Result<()> {
        let pos1 = self.get_position(prog1)?;
        let pos2 = self.get_position(prog2)?;
        self.programs.insert(prog1, pos2);
        self.programs.insert(prog2, pos1);
        Ok(())
    }

    fn get_position(&self, program: Program) -> Result<Position> {
        self.programs
            .get_by_left(&program)
            .copied()
            .with_context(|| format!("could not get position of program {:?}", program))
    }

    fn get_program(&self, position: Position) -> Result<Program> {
        self.programs
            .get_by_right(&position)
            .copied()
            .with_context(|| format!("could not get program at position {:?}", position))
    }

    /// Applies a DanceMap
    fn apply(&mut self, dance_map: &DanceMap) {
        let new_positions = self
            .programs
            .iter()
            .map(|(prog, pos)| (*prog, dance_map.get(pos)))
            .collect::<Vec<_>>();
        for (prog, pos) in new_positions {
            self.programs.insert(prog, pos);
        }
    }

    /// Applies a DanceMap n times
    fn apply_n(&mut self, dance_map: &DanceMap, n: usize) {
        if n == 0 {
            return;
        }
        if n % 2 == 0 {
            self.apply_n(&dance_map.square(), n / 2);
        } else {
            self.apply(dance_map);
            self.apply_n(dance_map, n - 1);
        }
    }

    /// Applies a DanceMap n times linearly
    fn apply_n_lin(&mut self, dance_map: &DanceMap, n: usize) {
        for _ in 0..n {
            self.apply(dance_map);
        }
    }

    /// Generates a single string that contains characters from 'a' to 'p'
    /// that encodes the order of the programs
    fn program_order(&self) -> String {
        let mut all: Vec<_> = self.programs.iter().collect();
        all.sort_by_key(|(_prog, pos)| **pos);
        all.iter().map(|(prog, _pos)| prog.0).collect()
    }
}

// TODO:
// There is a bug in this code.
// Partner is *not* a simple permutation, but those names should be actively swapped.
// Consider a solution using just two vectors to encode positions.

#[derive(Debug, PartialEq, Eq)]
enum DanceMove {
    Spin(u8),
    Exchange(Position, Position),
    Partner(Program, Program),
}

impl DanceMove {
    fn new(s: &str) -> Result<DanceMove> {
        match s.chars().next() {
            Some('s') => {
                let size = scanf!(s, "s{}", u8).context("could not get size of spin")?;
                Ok(DanceMove::Spin(size))
            }
            Some('x') => {
                let (a, b) =
                    scanf!(s, "x{}/{}", u8, u8).context("could not get positions for exchange")?;
                Ok(DanceMove::Exchange(Position(a), Position(b)))
            }
            Some('p') => {
                let (a, b) =
                    scanf!(s, "p{}/{}", char, char).context("could not get programs for swap")?;
                Ok(DanceMove::Partner(Program(a), Program(b)))
            }
            Some(_) => bail!("could not interpret dance {}", s),
            None => bail!("input is empty"),
        }
    }
}

/// DanceMap represents how Programs are "mapped" from one state of a Dance to another.
/// A program of position ii in the beginning state will be mapped to DanceMap[ii] at the ending state.
#[derive(Debug)]
struct DanceMap([u8; 5]);

impl DanceMap {
    fn new(p1: &Promenade, p2: &Promenade) -> Result<DanceMap> {
        let mut result = [0; 5];
        for ii in 0..5 {
            result[ii] = p2.get_position(p1.get_program(Position(ii as u8))?)?.0;
        }

        // Make sure result has unique elements
        let mut done = HashSet::new();
        for value in result {
            if done.contains(&value) {
                bail!("not unique values in dance positions");
            }
            done.insert(value);
        }

        Ok(DanceMap(result))
    }

    fn get(&self, position: &Position) -> Position {
        Position(self.0[position.0 as usize])
    }

    /// This works as follows
    /// ```none
    /// let d1 = DanceMap(p1, p2);
    /// let p3 = p2.apply(d1);
    /// // d1.square() == DanceMap::new(p1, p3)
    /// ```
    fn square(&self) -> DanceMap {
        let mut result = [0; 5];
        for ii in 0..5 {
            result[ii] = self.0[self.0[ii] as usize];
        }
        DanceMap(result)
    }
}

/// Mapping represents how a Promenade can go from one state to another.
struct Mapping {
    position_map: BiHashMap<Position, Position>,
    program_map: BiHashMap<Program, Program>,
}

impl Mapping {
    fn new(n: u8) {
        // TODO
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    if lines.len() != 1 {
        bail!("expected 1 line as input, got {} instead", lines.len())
    }
    let input = lines.into_iter().next().unwrap();
    let dance_moves = input
        .split(',')
        .map(|s| {
            DanceMove::new(s).with_context(|| format!("could not interpret '{}' as DanceMove", s))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not parse input")?;

    // Part 1: Perform all dances
    let mut promenade = Promenade::new();
    for dance in &dance_moves {
        let Ok(()) = promenade.perform(&dance) else {
            bail!("could not perform dance {:?}", dance)
        };
    }
    let ans1 = promenade.program_order();

    // Part 2: Perform a billion times
    let dance_map =
        DanceMap::new(&Promenade::new(), &promenade).context("could not create dance map")?;
    println!("{:?}", dance_map);
    // Suppose we run it again
    promenade.apply(&dance_map);
    println!("{}", promenade.program_order());

    let mut promenade = Promenade::new();
    promenade.apply_n(&dance_map, 1_000_000_000);
    let ans2 = promenade.program_order();

    Ok((ans1, ans2))
}
