use std::vec;

use anyhow::{bail, Context, Result};

use crate::util::vectors::split_and_trim_borrowed;

#[derive(Clone, Debug)]
struct Card {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

impl Card {
    fn new(line: &str) -> Result<Card> {
        let split_by_colon = line.split(": ").collect::<Vec<_>>();
        if split_by_colon.len() != 2 {
            bail!("could not split line by colon");
        }
        let (_card_index, numbers) = (split_by_colon[0], split_by_colon[1]);
        let split_numbers = numbers.split(" | ").collect::<Vec<_>>();
        if split_numbers.len() != 2 {
            bail!("could not split line by vertical bar")
        }
        let (winning_numbers, numbers_you_have) = (
            split_and_trim_borrowed(split_numbers[0], ' '),
            split_and_trim_borrowed(split_numbers[1], ' '),
        );
        let winning_numbers = winning_numbers
            .into_iter()
            .map(|s| s.parse::<u32>().ok())
            .collect::<Option<Vec<_>>>()
            .context("could not parse winning numbers properly")?;
        let numbers_you_have = numbers_you_have
            .into_iter()
            .map(|s| s.parse::<u32>().ok())
            .collect::<Option<Vec<_>>>()
            .context("could not parse numbers you have properly")?;

        Ok(Card { winning_numbers, numbers_you_have })
    }

    /// Count how many "winning numbers" there are per card,
    /// i.e., the number of common numbers between self.winning_numbers and "numbers you have".
    /// Set to u32 for convenience.
    fn count_matching_numbers(&self) -> u32 {
        // We know there are at most 100 cards
        let mut map = [0; 100];
        for vv in &self.winning_numbers {
            map[*vv as usize] += 1;
        }
        for vv in &self.numbers_you_have {
            map[*vv as usize] += 1;
        }

        let mut result = 0;
        for vv in map {
            if vv == 2 {
                result += 1;
            }
        }
        result
    }
}

fn solve_part_1(cards: &Vec<Card>) -> Result<String> {
    let matching_number_counts =
        cards.iter().map(|card| card.count_matching_numbers());

    let points = matching_number_counts.filter_map(|matching_number_count| {
        if matching_number_count == 0 {
            None
        } else {
            Some(u32::pow(2, matching_number_count - 1))
        }
    });
    Ok(points.sum::<u32>().to_string())
}

fn solve_part_2(cards: &Vec<Card>) -> Result<String> {
    let cards_len = cards.len();
    let matching_number_counts = cards
        .iter()
        .map(|card| card.count_matching_numbers())
        .collect::<Vec<_>>();
    let mut scratchcards = vec![1; cards_len];
    for ii in 0..cards_len {
        let current_matching = matching_number_counts[ii];
        for jj in (ii + 1)..(ii + 1 + current_matching as usize) {
            if jj > cards_len {
                break;
            }
            scratchcards[jj] += scratchcards[ii];
        }
    }

    Ok(scratchcards.iter().sum::<u32>().to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let cards = lines
        .into_iter()
        .map(|line| {
            Card::new(&line)
                .with_context(|| format!("could not parse `{}`", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("could not parse input")?;

    Ok((solve_part_1(&cards), solve_part_2(&cards)))
}
