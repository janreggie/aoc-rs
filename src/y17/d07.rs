use std::collections::HashMap;

use crate::util::vectors::odd_one_out_index;
use anyhow::{Context, Result};
use sscanf::scanf;

#[derive(Debug, Clone)]
struct ProgramGraph {
    length: usize,
    names: Vec<String>,
    weights: Vec<u32>,
    children: Vec<Vec<usize>>, // children[ii] == indices of children of names[ii]
    bottom_ind: usize,         // the index of the "root"
}

impl ProgramGraph {
    pub fn new(lines: &Vec<String>) -> Result<ProgramGraph> {
        // First, parse all programs without children data
        let mut names = vec![];
        let mut weights = vec![];
        let mut name_to_ind = HashMap::new();
        let mut children_map = HashMap::new();
        for line in lines {
            let mut split_line = line.split(" -> ");
            let (name, weight) = scanf!(
                split_line.nth(0).context("could not get name and weight")?,
                "{} ({})",
                String,
                u32
            )
            .context("could not parse name and weight")?;
            let children = {
                if let Some(subprogs) = split_line.nth(0) {
                    subprogs.split(", ").map(|s| s.to_string()).collect()
                } else {
                    vec![]
                }
            };
            names.push(name.clone());
            weights.push(weight);
            name_to_ind.insert(name.clone(), names.len() - 1);
            children_map.insert(name, children);
        }

        // Then, implement children
        let length = names.len();
        let mut children = vec![vec![]; length];
        for (parent_name, children_names) in children_map {
            let parent_ind = name_to_ind[&parent_name];
            let children_inds: Vec<_> = children_names.iter().map(|s| name_to_ind[s]).collect();
            children[parent_ind] = children_inds;
        }

        // Then, look for the "bottom node"
        let mut is_colored = vec![false; length];
        for ii in 0..length {
            let children = &children[ii];
            for child in children {
                is_colored[*child] = true;
            }
        }
        let bottom_ind = is_colored
            .iter()
            .enumerate()
            .filter(|(_, &r)| !r)
            .map(|(ii, _)| ii)
            .next()
            .context("invalid graph: could not get root index")?;

        Ok(ProgramGraph {
            length,
            names,
            weights,
            children,
            bottom_ind,
        })
    }

    /// Returns the weights of the programs and all programs above itself
    fn cumulative_weights(&self) -> Vec<u32> {
        let mut cumulative_weights = vec![0; self.length];
        fn populate_weights(
            ind: usize,
            cum_weights: &mut Vec<u32>,
            children: &Vec<Vec<usize>>,
            weights: &Vec<u32>,
        ) -> u32 {
            for child in &children[ind] {
                cum_weights[ind] += populate_weights(*child, cum_weights, children, weights);
            }
            cum_weights[ind] += weights[ind];
            cum_weights[ind]
        }
        populate_weights(
            self.bottom_ind,
            &mut cumulative_weights,
            &self.children,
            &self.weights,
        );
        cumulative_weights
    }

    /// Returns the index of the offending program and its ideal weight
    fn offending_program(&self) -> (usize, u32) {
        let cumulative_weights = self.cumulative_weights();

        // get_weird_one_out returns two things:
        // - the sibling indices in which the weight is a bit off
        // - the index within those sibling indices which is too heavy/light
        fn get_weird_one_out<'a>(
            inds: &'a Vec<usize>,
            cum_weights: &Vec<u32>,
            children: &'a Vec<Vec<usize>>,
        ) -> Option<(&'a Vec<usize>, usize)> {
            if inds.len() == 0 {
                return None;
            }
            let weights: Vec<_> = inds.iter().map(|&ii| cum_weights[ii]).collect();
            if let Some(weird_index) = odd_one_out_index(&weights) {
                let investigating_weird =
                    get_weird_one_out(&children[inds[weird_index]], cum_weights, children);
                Some(investigating_weird.unwrap_or((inds, weird_index)))
            } else {
                None
            }
        }

        let (weird_siblings, weird_sibling_ind) = get_weird_one_out(
            &self.children[self.bottom_ind],
            &cumulative_weights,
            &self.children,
        )
        .unwrap();

        let offending_program_ind = weird_siblings[weird_sibling_ind];
        let normal_program_ind = if weird_sibling_ind > 0 {
            weird_siblings[weird_sibling_ind - 1]
        } else {
            weird_siblings[weird_sibling_ind + 1]
        };
        let offending_cum_weight = cumulative_weights[offending_program_ind];
        let normal_cum_weight = cumulative_weights[normal_program_ind];

        (
            offending_program_ind,
            self.weights[offending_program_ind] + normal_cum_weight - offending_cum_weight,
        )
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let program_graph = ProgramGraph::new(&lines).context("could not create program graph")?;

    let ans1 = program_graph.names[program_graph.bottom_ind].clone();
    let (_, ans2) = program_graph.offending_program();
    let ans2 = ans2.to_string();
    Ok((ans1, ans2))
}
