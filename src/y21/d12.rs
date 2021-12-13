use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    paths: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn new(lines: Vec<String>) -> Result<Graph> {
        let mut paths = HashMap::new();

        for line in lines {
            let mut nodes = vectors::split_and_trim(&line, '-');
            if nodes.len() != 2 {
                bail!("expected {} to be split into 2, got {}", line, nodes.len());
            }

            let n1 = nodes.pop().unwrap();
            let n2 = nodes.pop().unwrap();
            let p = paths.entry(n1.clone()).or_insert(HashSet::new());
            p.insert(n2.clone());
            let p = paths.entry(n2.clone()).or_insert(HashSet::new());
            p.insert(n1.clone());
        }

        Ok(Graph { paths })
    }

    fn paths_from_start_to_end(&self) -> usize {
        // Okay, let's go from here to there...
        0
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let graph = Graph::new(lines).context("could not create graph")?;
    dbg!(&graph);

    let ans1 = graph.paths_from_start_to_end();

    Ok((ans1.to_string(), String::from("unimplemented")))
}
