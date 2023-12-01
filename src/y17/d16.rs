use anyhow::{bail, Context, Result};
use bimap::BiHashMap;
use sscanf::scanf;

#[derive(Debug)]
struct Promenade {
    programs: BiHashMap<char, u8>, // Program <-> Position
    size: u8,
}

impl Promenade {
    fn new(n: u8) -> Promenade {
        let mut programs = BiHashMap::new();
        for ii in 0..n {
            programs.insert((0x61 + ii) as char, ii);
        }
        Promenade { programs, size: n }
    }

    /// Applies a mapping into a Promenade.
    /// mapping.size must equal promenade.size.
    fn apply(&mut self, mapping: &Mapping) {
        assert!(self.size == mapping.size);
        let pairs = self
            .programs
            .iter()
            .map(|(&prog, &pos)| {
                (mapping.program(prog).unwrap(), mapping.position(pos).unwrap())
            })
            .collect::<Vec<_>>();
        for (prog, pos) in pairs {
            self.programs.insert(prog, pos);
        }
    }

    /// Generates a single string that contains characters from 'a' to 'p'
    /// that encodes the order of the programs
    fn program_order(&self) -> String {
        let mut all: Vec<_> = self.programs.iter().collect();
        all.sort_by_key(|(_prog, pos)| **pos);
        all.iter().map(|(prog, _pos)| **prog).collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum DanceMove {
    Spin(u8),
    Exchange(u8, u8),
    Partner(char, char),
}

impl DanceMove {
    fn new(s: &str) -> Result<DanceMove> {
        match s.chars().next() {
            Some('s') => {
                let size = scanf!(s, "s{}", u8)
                    .context("could not get size of spin")?;
                Ok(DanceMove::Spin(size))
            }
            Some('x') => {
                let (a, b) = scanf!(s, "x{}/{}", u8, u8)
                    .context("could not get positions for exchange")?;
                Ok(DanceMove::Exchange(a, b))
            }
            Some('p') => {
                let (a, b) = scanf!(s, "p{}/{}", char, char)
                    .context("could not get programs for swap")?;
                Ok(DanceMove::Partner(a, b))
            }
            Some(_) => bail!("could not interpret dance {}", s),
            None => bail!("input is empty"),
        }
    }
}

/// Mapping represents how a Promenade can go from one state to another.
#[derive(Debug, Clone)]
struct Mapping {
    position_map: BiHashMap<u8, u8>,
    program_map: BiHashMap<char, char>,
    size: u8,
}

impl Mapping {
    fn new(n: u8) -> Mapping {
        let mut position_map = BiHashMap::new();
        let mut program_map = BiHashMap::new();
        for ii in 0..n {
            position_map.insert(ii, ii);
            program_map.insert((0x61 + ii) as char, (0x61 + ii) as char);
        }

        Mapping { position_map, program_map, size: n }
    }

    fn position(&self, old_pos: u8) -> Option<u8> {
        self.position_map.get_by_left(&old_pos).copied()
    }

    fn program(&self, old_prog: char) -> Option<char> {
        self.program_map.get_by_left(&old_prog).copied()
    }

    fn add(&mut self, dance_move: &DanceMove) -> Result<()> {
        match dance_move {
            DanceMove::Spin(s) => self.spin(*s),
            DanceMove::Exchange(a, b) => self.exchange(*a, *b),
            DanceMove::Partner(a, b) => self.partner(*a, *b),
        }
    }

    fn spin(&mut self, s: u8) -> Result<()> {
        if s >= self.size {
            bail!("spin {} too big", s)
        }
        let pairs = self
            .position_map
            .iter()
            .map(|(&ll, &rr)| (ll, (rr + s) % self.size))
            .collect::<Vec<_>>();

        for (ll, rr) in pairs {
            self.position_map.insert(ll, rr);
        }
        Ok(())
    }

    fn exchange(&mut self, a: u8, b: u8) -> Result<()> {
        if a >= self.size || b >= self.size {
            bail!("exchange {},{} unsupported", a, b)
        }
        let left_a = self
            .position_map
            .get_by_right(&a)
            .copied()
            .context("a's source unknown")?;
        let left_b = self
            .position_map
            .get_by_right(&b)
            .copied()
            .context("b's source unknown")?;
        self.position_map.insert(left_a, b);
        self.position_map.insert(left_b, a);
        Ok(())
    }

    fn partner(&mut self, a: char, b: char) -> Result<()> {
        if (a as u8) >= (self.size + 0x61) || (b as u8) >= (self.size + 0x61) {
            bail!("partner {},{} unsupported", a, b)
        }
        let left_a = self
            .program_map
            .get_by_right(&a)
            .copied()
            .context("a's source unknown")?;
        let left_b = self
            .program_map
            .get_by_right(&b)
            .copied()
            .context("b's source unknown")?;
        self.program_map.insert(left_a, b);
        self.program_map.insert(left_b, a);
        Ok(())
    }

    fn square(&self) -> Mapping {
        self.apply(self)
    }

    fn apply(&self, m2: &Mapping) -> Mapping {
        assert!(self.size == m2.size);

        let mut position_map = BiHashMap::new();
        for left_pos in 0..self.size {
            let right_pos =
                m2.position(self.position(left_pos).unwrap()).unwrap();
            position_map.insert(left_pos, right_pos);
        }
        let mut program_map = BiHashMap::new();
        for left_prog in 0..self.size {
            let left_prog = (left_prog + 0x61) as char;
            let right_prog =
                m2.program(self.program(left_prog).unwrap()).unwrap();
            program_map.insert(left_prog, right_prog);
        }

        Mapping { position_map, program_map, size: self.size }
    }

    fn pow(&self, n: usize) -> Mapping {
        if n == 0 {
            Mapping::new(self.size)
        } else if n == 1 {
            self.clone()
        } else if n % 2 == 0 {
            self.square().pow(n / 2)
        } else {
            self.square().pow(n / 2).apply(self)
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    if lines.len() != 1 {
        bail!("expected 1 line as input, got {} instead", lines.len())
    }
    let input = lines.into_iter().next().unwrap();
    let dance_moves = input
        .split(',')
        .map(|s| {
            DanceMove::new(s).with_context(|| {
                format!("could not interpret '{}' as DanceMove", s)
            })
        })
        .collect::<Result<Vec<_>>>()
        .context("could not parse input")?;

    // Part 1: Perform all dances
    let mut promenade = Promenade::new(16);
    let mut mapping = Mapping::new(16);
    for dance_move in &dance_moves {
        let Ok(_) = mapping.add(dance_move) else {
            bail!("could not perform dance move {:?}", dance_move)
        };
    }
    promenade.apply(&mapping);
    let ans1 = Ok(promenade.program_order());

    // Part 2: Perform a billion times
    let mapping = mapping.pow(1_000_000_000);
    let mut promenade = Promenade::new(16);
    promenade.apply(&mapping);
    let ans2 = Ok(promenade.program_order());

    Ok((ans1, ans2))
}
