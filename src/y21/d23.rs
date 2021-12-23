use anyhow::{bail, Context, Result};
use priority_queue::DoublePriorityQueue;
use sscanf::scanf;
use std::collections::HashSet;
use std::fmt;

// This is going to be the shittiest graph that I will be making.
// But it will be a graph that will work.

/// Graph represents the current state of the nodes
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Graph {
    // The four layers of the Graph.
    // Recal that mid[0] can "go" to top[1] or top[2].
    top: [Val; 7],
    mid: [Val; 4],
    bot: [Val; 4],
}

impl Graph {
    /// Reads the input, lazily and without regard if the input is invalid
    fn new(input: Vec<String>) -> Result<Graph> {
        if input.len() != 5 {
            bail!("expected input to be 5 lines, got {} instead", input.len())
        }

        let first_line = &input[2];
        let second_line = &input[3];
        let (m1, m2, m3, m4) = scanf!(first_line, "###{}#{}#{}#{}###", char, char, char, char)
            .context(format!("could not parse first line `{}`", first_line))?;
        let (b1, b2, b3, b4) = scanf!(second_line, "  #{}#{}#{}#{}#", char, char, char, char)
            .context(format!("could not parse second line `{}`", second_line))?;

        fn p(c: char) -> Option<Val> {
            match c {
                'A' => Some(Val::Amber),
                'B' => Some(Val::Bronze),
                'C' => Some(Val::Copper),
                'D' => Some(Val::Desert),
                _ => None,
            }
        }

        let top = [Val::Nil; 7];
        let mid = [
            p(m1).context("could not parse m1")?,
            p(m2).context("could not parse m2")?,
            p(m3).context("could not parse m3")?,
            p(m4).context("could not parse m4")?,
        ];
        let bot = [
            p(b1).context("could not parse b1")?,
            p(b2).context("could not parse b2")?,
            p(b3).context("could not parse b3")?,
            p(b4).context("could not parse b4")?,
        ];

        Ok(Graph { top, mid, bot })
    }

    /// Create all possible permutations of the next Graph, with respective costs
    fn next(&self) -> Vec<(Graph, u32)> {
        vec![self.next_top(), self.next_mid(), self.next_bot()].concat()
    }

    fn next_top(&self) -> Vec<(Graph, u32)> {
        let mut result = Vec::new();

        // Nodes on the top row only move with each other
        if self.top[0] != Val::Nil && self.top[1] == Val::Nil {
            let node = self.top[0];
            let mut next = *self;
            next.top[1] = node;
            next.top[0] = Val::Nil;
            result.push((next, node.cost()))
        }
        for ii in 1..6 {
            if self.top[ii] != Val::Nil {
                let node = self.top[ii];
                if self.top[ii - 1] == Val::Nil {
                    let mut next = *self;
                    next.top[ii - 1] = node;
                    next.top[ii] = Val::Nil;
                    result.push((next, node.cost() * if ii == 1 { 1 } else { 2 }))
                }
                if self.top[ii + 1] == Val::Nil {
                    let mut next = *self;
                    next.top[ii + 1] = node;
                    next.top[ii] = Val::Nil;
                    result.push((next, node.cost() * if ii == 5 { 1 } else { 2 }))
                }
            }
        }
        if self.top[6] != Val::Nil && self.top[5] == Val::Nil {
            let node = self.top[6];
            let mut next = *self;
            next.top[5] = node;
            next.top[6] = Val::Nil;
            result.push((next, node.cost()))
        }

        // Nodes on the top row moving to the bottom row
        for ii in 0..4 {
            // Move to the "right"
            let node = self.top[ii + 1];
            if node != Val::Nil && self.mid[ii] == Val::Nil && node.expected_col() == ii {
                let mut next = *self;
                next.mid[ii] = node;
                next.top[ii + 1] = Val::Nil;
                result.push((next, node.cost() * 2));
            }
        }
        for ii in 0..4 {
            // Move to the "left"
            let node = self.top[ii + 2];
            if node != Val::Nil && self.mid[ii] == Val::Nil && node.expected_col() == ii {
                let mut next = *self;
                next.mid[ii] = node;
                next.top[ii + 2] = Val::Nil;
                result.push((next, node.cost() * 2));
            }
        }

        result
    }

    fn next_mid(&self) -> Vec<(Graph, u32)> {
        let mut result = Vec::new();
        for ii in 0..4 {
            let node = self.mid[ii];
            if node == Val::Nil {
                continue;
            }

            // Move to the top (left)
            if self.top[ii + 1] == Val::Nil {
                let mut next = *self;
                next.top[ii + 1] = node;
                next.mid[ii] = Val::Nil;
                result.push((next, node.cost() * 2));
            }

            // Move to the top (right)
            if self.top[ii + 2] == Val::Nil {
                let mut next = *self;
                next.top[ii + 2] = node;
                next.mid[ii] = Val::Nil;
                result.push((next, node.cost() * 2));
            }

            // Move to the bottom
            if self.bot[ii] == Val::Nil {
                let mut next = *self;
                next.bot[ii] = node;
                next.mid[ii] = Val::Nil;
                result.push((next, node.cost()));
            }
        }

        result
    }

    fn next_bot(&self) -> Vec<(Graph, u32)> {
        let mut result = Vec::new();
        for ii in 0..4 {
            let node = self.bot[ii];
            if node != Val::Nil && self.mid[ii] == Val::Nil {
                let mut next = *self;
                next.mid[ii] = node;
                next.bot[ii] = Val::Nil;
                result.push((next, node.cost()));
            }
        }
        result
    }

    fn is_solved(&self) -> bool {
        for t in self.top {
            if t != Val::Nil {
                return false;
            }
        }
        for ii in 0..4 {
            if self.mid[ii].expected_col() != ii {
                return false;
            }
            if self.bot[ii].expected_col() != ii {
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "#############")?;
        writeln!(
            f,
            "#{}{}.{}.{}.{}.{}{}#",
            self.top[0],
            self.top[1],
            self.top[2],
            self.top[3],
            self.top[4],
            self.top[5],
            self.top[6]
        )?;
        writeln!(
            f,
            "###{}#{}#{}#{}###",
            self.mid[0], self.mid[1], self.mid[2], self.mid[3],
        )?;
        writeln!(
            f,
            "  #{}#{}#{}#{}#",
            self.bot[0], self.bot[1], self.bot[2], self.bot[3]
        )?;
        write!(f, "  #########")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Val {
    Amber,
    Bronze,
    Copper,
    Desert,
    Nil, // Empty space
}

impl Val {
    fn cost(&self) -> u32 {
        use Val::*;
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
            Nil => panic!("could not get cost for Nil"),
        }
    }

    fn expected_col(&self) -> usize {
        use Val::*;
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
            Nil => panic!("could not get column for Nil"),
        }
    }
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Val::Amber => 'A',
                Val::Bronze => 'B',
                Val::Copper => 'C',
                Val::Desert => 'D',
                Val::Nil => '.',
            }
        )
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let graph = Graph::new(lines).context("could not create graph")?;
    let mut pq: DoublePriorityQueue<(Graph, Vec<(Graph, u32)>), u32> = DoublePriorityQueue::new();
    pq.push((graph, vec![]), 0);
    let mut seen = HashSet::new();
    seen.insert(graph);

    let mut ans1 = 0;
    while let Some(((gg, hist), val)) = pq.pop_min() {
        if gg.is_solved() {
            ans1 = val;
            for hp in hist {
                eprintln!("Score:{}\n{}", hp.1, hp.0)
            }
            break;
        }

        for (ng, nv) in gg.next() {
            if !seen.contains(&ng) {
                seen.insert(ng);
                let mut nh = hist.clone();
                nh.push((gg, val));
                pq.push((ng, nh), nv + val);
            }
        }
    }

    // Do things with graph
    Ok((ans1.to_string(), String::from("undefined")))
}
