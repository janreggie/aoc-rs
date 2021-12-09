use crate::util::vectors;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct BingoCard {
    nums: Vec<Vec<u32>>,
    shaded: Vec<Vec<bool>>,
}

impl BingoCard {
    fn new(lines: &Vec<String>) -> Result<BingoCard> {
        if lines.len() != 5 {
            bail!(
                "expected number of lines to be 5, got {} instead",
                lines.len()
            );
        }

        let mut result = BingoCard {
            nums: Vec::new(),
            shaded: vec![vec![false; 5]; 5],
        };
        for line in lines {
            let row = vectors::split_and_trim(line, ' ');
            let row = vectors::from_strs::<u32>(&row)
                .context(format!("could not parse line `{}`", line))?;
            result.nums.push(row);
        }

        Ok(result)
    }

    fn has_won(self: &BingoCard) -> bool {
        // Check rows
        for rr in 0..5 {
            let mut row_all_shaded = true;
            for cc in 0..5 {
                if !self.shaded[rr][cc] {
                    row_all_shaded = false;
                    break;
                }
            }
            if row_all_shaded {
                return true;
            }
        }

        // Check cols
        for cc in 0..5 {
            let mut col_all_shaded = true;
            for rr in 0..5 {
                if !self.shaded[rr][cc] {
                    col_all_shaded = false;
                    break;
                }
            }
            if col_all_shaded {
                return true;
            }
        }

        false
    }

    fn shade(&mut self, num: u32) -> Option<u32> {
        // Find num in rows and cols.
        let mut has_shaded = false;
        'outer: for rr in 0..5 {
            for cc in 0..5 {
                if self.nums[rr][cc] == num {
                    self.shaded[rr][cc] = true;
                    has_shaded = true;
                    break 'outer;
                }
            }
        }

        if !has_shaded || !self.has_won() {
            return None;
        }

        // By this point, the Card "won".
        // Let's calculate.
        let mut sum_of_unmarked = 0;
        for rr in 0..5 {
            for cc in 0..5 {
                if !self.shaded[rr][cc] {
                    sum_of_unmarked += self.nums[rr][cc];
                }
            }
        }
        Some(sum_of_unmarked * num)
    }

    fn clear(&mut self) {
        for rr in 0..5 {
            for cc in 0..5 {
                self.shaded[rr][cc] = false;
            }
        }
    }
}

#[derive(Debug)]
struct BingoCards {
    cards: Vec<BingoCard>,

    /// All Bingo Cards in `lookup[x]` contain the number `x`.
    lookup: HashMap<u32, Vec<usize>>,
}

impl BingoCards {
    fn new(groups: &Vec<Vec<String>>) -> Result<BingoCards> {
        let mut result = BingoCards {
            cards: Vec::new(),
            lookup: HashMap::new(),
        };

        for group in groups {
            let card =
                BingoCard::new(group).context(format!("could not interpret group {:?}", group))?;

            // Update lookup to include items from cards
            for row in &card.nums {
                for num in row {
                    let m = result.lookup.entry(*num).or_insert(Vec::new());
                    m.push(result.cards.len());
                }
            }
            result.cards.push(card);
        }

        Ok(result)
    }

    fn clear(&mut self) {
        for card in &mut self.cards {
            card.clear();
        }
    }

    /// All cards will have `num` shaded.
    /// Returns either None, or a list of pairs,
    /// in which each pair contains the index of the bingo card that won,
    /// as well as the score of said bingo card.
    ///
    /// It doesn't return the cards which have already won beforehand.
    ///
    fn shade(&mut self, num: u32) -> Option<Vec<(usize, u32)>> {
        if !self.lookup.contains_key(&num) {
            return None;
        }

        let inds = &self.lookup[&num];
        let mut result = Vec::new();
        for ind in inds {
            let card = &mut self.cards[*ind];

            // If already won, don't store to result, but still shade
            if card.has_won() {
                card.shade(num);
                continue;
            }

            if let Some(res) = card.shade(num) {
                result.push((*ind, res));
            }
        }

        if result.len() != 0 {
            Some(result)
        } else {
            None
        }
    }
}

pub fn solve(lines: Vec<String>) -> Result<(String, String)> {
    let mut groups = vectors::group(lines);
    if groups.len() < 2 {
        bail!("input must have at least two groups of continguous lines");
    }

    let mut bingo_numbers = groups.swap_remove(0);
    if bingo_numbers.len() != 1 {
        bail!("group[0] must have length 1, got {}", bingo_numbers.len());
    }
    let bingo_numbers = bingo_numbers.remove(0); // Take the first row of the group
    let bingo_numbers = vectors::split_and_trim(&bingo_numbers, ',');
    let bingo_numbers = vectors::from_strs::<u32>(&bingo_numbers)
        .context("could not format bingo nunmbers properly")?;

    let mut bingo_cards =
        BingoCards::new(&groups).context("could not format bingo cards properly")?;

    // Part 1: First to win
    let mut ans1 = 0;
    for num in &bingo_numbers {
        let result = bingo_cards.shade(*num);
        if let Some(winners) = result {
            ans1 = winners[0].1;
            break;
        }
    }

    // Part 2: Last to win
    bingo_cards.clear();
    let mut all_card_indices = HashSet::new();
    for ii in 0..bingo_cards.cards.len() {
        all_card_indices.insert(ii);
    }

    let mut ans2 = 0;
    for num in &bingo_numbers {
        let result = bingo_cards.shade(*num);
        if let Some(winners) = result {
            for vv in &winners {
                all_card_indices.remove(&vv.0);
            }
            if all_card_indices.is_empty() {
                ans2 = winners[0].1;
                break;
            }
        }
    }

    Ok((ans1.to_string(), ans2.to_string()))
}
