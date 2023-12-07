use std::cmp::Ordering;

use anyhow::{bail, Context, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    /// Returns a card from 2~9, JT, J, Q, K, and A.
    fn new(c: char) -> Option<Card> {
        match c {
            '2' => Some(Card::N2),
            '3' => Some(Card::N3),
            '4' => Some(Card::N4),
            '5' => Some(Card::N5),
            '6' => Some(Card::N6),
            '7' => Some(Card::N7),
            '8' => Some(Card::N8),
            '9' => Some(Card::N9),
            'T' => Some(Card::T),
            'J' => Some(Card::J),
            'Q' => Some(Card::Q),
            'K' => Some(Card::K),
            'A' => Some(Card::A),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    /// Returns a Hand from an input e.g., "AA8AA"
    fn new(input: &str) -> Option<Hand> {
        if input.len() != 5 {
            return None;
        }

        let cards =
            input.chars().map(|c| Card::new(c)).collect::<Option<Vec<_>>>()?;
        // We know that cards is length 5 so unwrap shouldn't panic
        Some(Hand { cards: cards.try_into().unwrap() })
    }

    fn get_type(&self) -> HandType {
        let mut cards = self.cards;
        cards.sort();

        if cards[0] == cards[4] {
            return HandType::FiveOfAKind;
        }
        if cards[0] == cards[3] || cards[1] == cards[4] {
            return HandType::FourOfAKind;
        }
        if cards[0] == cards[2] || cards[2] == cards[4] {
            if cards[0] == cards[1] && cards[3] == cards[4] {
                return HandType::FullHouse;
            }
            return HandType::ThreeOfAKind;
        }
        if cards[1] == cards[3] {
            return HandType::ThreeOfAKind;
        }
        let (b1, b2, b3, b4) = (
            cards[0] == cards[1],
            cards[1] == cards[2],
            cards[2] == cards[3],
            cards[3] == cards[4],
        );
        if b1 && b3 || b1 && b4 || b2 && b4 {
            return HandType::TwoPair;
        }
        if b1 || b2 || b3 || b4 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }

    /// Returns HandType when considering jokers
    fn get_type_with_joker(&self) -> HandType {
        let hand_type = self.get_type();
        let joker_count = self.cards.iter().filter(|c| **c == Card::J).count();
        if joker_count == 0 {
            hand_type
        } else {
            match hand_type {
                HandType::FiveOfAKind => hand_type,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FullHouse => {
                    // There are either 2 or 3 jokers. Either way, all jokers can turn into the other cards
                    HandType::FiveOfAKind
                }
                HandType::ThreeOfAKind => {
                    // Either there's one joker, or three.
                    // If there's one joker, it can turn to the other three cards---and vice versa.
                    HandType::FourOfAKind
                }
                HandType::TwoPair => {
                    if joker_count == 2 {
                        // AAJJB -> AAAAB
                        HandType::FourOfAKind
                    } else {
                        // AABBJ -> AABBB
                        HandType::FullHouse
                    }
                }
                HandType::OnePair => {
                    // AAJBC -> AAABC
                    // JJABC -> AAABC
                    HandType::ThreeOfAKind
                }
                HandType::HighCard => HandType::OnePair,
            }
        }
    }

    fn cmp_1(&self, other: &Self) -> Ordering {
        let ordering = self.get_type().cmp(&other.get_type());
        if ordering.is_eq() {
            self.cards.cmp(&other.cards)
        } else {
            ordering
        }
    }

    fn cmp_2(&self, other: &Self) -> Ordering {
        let ordering =
            self.get_type_with_joker().cmp(&other.get_type_with_joker());
        if ordering.is_eq() {
            for ii in 0..5 {
                let (lhs, rhs) = (self.cards[ii], other.cards[ii]);
                if lhs == rhs {
                    continue;
                }
                return match (lhs, rhs) {
                    (Card::J, _) => Ordering::Less,
                    (_, Card::J) => Ordering::Greater,
                    (_, _) => lhs.cmp(&rhs),
                };
            }
            Ordering::Equal
        } else {
            ordering
        }
    }
}

#[test]
fn test_hand_get_type() {
    let hands = vec![
        (Hand::new("32T3K").unwrap(), HandType::OnePair),
        (Hand::new("KK677").unwrap(), HandType::TwoPair),
        (Hand::new("KTJJT").unwrap(), HandType::TwoPair),
        (Hand::new("T55J5").unwrap(), HandType::ThreeOfAKind),
        (Hand::new("QQQJA").unwrap(), HandType::ThreeOfAKind),
    ];
    for (hand, hand_type) in hands {
        assert_eq!(
            hand.get_type(),
            hand_type,
            "Test if {:?} is of type {:?}",
            &hand,
            &hand_type
        );
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone)]
struct HandAndBid {
    hand: Hand,
    bid: u32,
}

impl HandAndBid {
    fn new(input: &str) -> Result<HandAndBid> {
        let split_input = input.split(' ').collect::<Vec<_>>();
        if split_input.len() != 2 {
            bail!("input should be in two")
        }

        let (hand, bid) = (split_input[0], split_input[1]);
        let hand = Hand::new(hand).context("could not parse hand")?;
        let bid = bid.parse::<u32>().context("could not parse bid")?;
        Ok(HandAndBid { hand, bid })
    }
}

impl PartialEq for HandAndBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}
impl Eq for HandAndBid {}

#[test]
fn test_hand_ordering() {
    let hands_and_bids = vec![
        HandAndBid::new("32T3K 765").unwrap(),
        HandAndBid::new("KTJJT 220").unwrap(),
        HandAndBid::new("KK677 28").unwrap(),
        HandAndBid::new("T55J5 684").unwrap(),
        HandAndBid::new("QQQJA 483").unwrap(),
    ];
    assert_eq!(
        hands_and_bids[0].hand.cmp_1(&hands_and_bids[1].hand),
        Ordering::Less,
        "Check if {:?} < {:?}",
        hands_and_bids[0],
        hands_and_bids[1]
    );
    assert_eq!(
        hands_and_bids[1].hand.cmp_1(&hands_and_bids[2].hand),
        Ordering::Less,
        "Check if {:?} < {:?}",
        hands_and_bids[1],
        hands_and_bids[2]
    );
    assert_eq!(
        hands_and_bids[2].hand.cmp_1(&hands_and_bids[3].hand),
        Ordering::Less,
        "Check if {:?} < {:?}",
        hands_and_bids[2],
        hands_and_bids[3]
    );
    assert_eq!(
        hands_and_bids[3].hand.cmp_1(&hands_and_bids[4].hand),
        Ordering::Less,
        "Check if {:?} < {:?}",
        hands_and_bids[3],
        hands_and_bids[4]
    );
}

fn solve_part_1(hands_and_bids: Vec<HandAndBid>) -> Result<String> {
    let mut hands_and_bids = hands_and_bids;
    hands_and_bids.sort_by(|lhs, rhs| lhs.hand.cmp_1(&rhs.hand));

    Ok(hands_and_bids
        .into_iter()
        .zip(1..)
        .map(|(hand_and_bid, rank)| rank * hand_and_bid.bid)
        .sum::<u32>()
        .to_string())
}

fn solve_part_2(hands_and_bids: Vec<HandAndBid>) -> Result<String> {
    let mut hands_and_bids = hands_and_bids;
    hands_and_bids.sort_by(|lhs, rhs| lhs.hand.cmp_2(&rhs.hand));

    Ok(hands_and_bids
        .into_iter()
        .zip(1..)
        .map(|(hand_and_bid, rank)| rank * hand_and_bid.bid)
        .sum::<u32>()
        .to_string())
}

pub fn solve(lines: Vec<String>) -> Result<(Result<String>, Result<String>)> {
    let hands_and_bids = lines
        .iter()
        .map(|line| {
            HandAndBid::new(line)
                .with_context(|| format!("cannot parse line `{}`", line))
        })
        .collect::<Result<Vec<_>>>()
        .context("cannot parse input")?;
    Ok((solve_part_1(hands_and_bids.clone()), solve_part_2(hands_and_bids)))
}
