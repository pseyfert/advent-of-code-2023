// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;

use std::{io::BufRead, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
enum Card {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq)]
struct NotAHand;
#[derive(Debug, PartialEq, Eq)]
struct NotACard;

impl FromStr for Card {
    type Err = NotACard;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert_eq!(s.len(), 1);
        match s.chars().nth(0) {
            Some(c) => Card::try_from(c).or(Err(NotACard)),
            None => Err(NotACard),
        }
    }
}

impl TryFrom<char> for Card {
    type Error = NotACard;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Number(10)),
            c => {
                let n = c.to_digit(10);
                match n {
                    Some(2..=9) => Ok(Card::Number(n.unwrap() as u8)),
                    _ => Err(NotACard),
                }
            }
        }
    }
}

impl FromStr for Hand {
    type Err = NotAHand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let (Some(cards), Some(bid), None) = (iter.next(), iter.next(), iter.next()) else {
            return Err(NotAHand);
        };
        assert_eq!(5, cards.len());
        let tmp: [Card; 5] = cards
            .chars()
            .map(|c| Card::try_from(c).unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Hand {
            cards: tmp,
            bid: bid.parse::<u64>().unwrap(),
        })
    }
}

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Value {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn value_joker(h: &Hand) -> Value {
    let tmp: Vec<_> = h
        .cards
        .iter()
        .filter_map(|c| match c {
            Card::Number(1) => None,
            c => Some(c.clone()),
        })
        .sorted()
        .collect();
    let grml: Vec<_> = tmp
        .iter()
        .group_by(|c| *c)
        .into_iter()
        .map(|(_k, g)| g.into_iter().count())
        .sorted()
        .collect();
    match grml.len() {
        0 | 1 => Value::FiveOfAKind,
        2 => {
            if *grml.get(0).unwrap() == 1 {
                Value::FourOfAKind
            } else {
                Value::FullHouse
            }
        }
        3 => {
            if *grml.get(1).unwrap() == 2 {
                Value::TwoPairs
            } else {
                Value::ThreeOfAKind
            }
        }
        4 => Value::OnePair,
        5 => Value::HighCard,
        _ => panic!("more than 5 cards?"),
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        match value_joker(self).cmp(&value_joker(other)) {
            std::cmp::Ordering::Equal => {
                for (lc, rc) in self.cards.iter().zip(&other.cards) {
                    if *lc != *rc {
                        return lc.cmp(rc);
                    }
                }
                std::cmp::Ordering::Equal
            }
            o => o,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u64,
}

fn main() {
    let line_iter =
        std::io::BufReader::new(std::fs::File::open(just_a_filename::Cli::parse().path).unwrap())
            .lines()
            .map(|l| l.unwrap());
    let h: Vec<_> = line_iter
        .map(|l| Hand::from_str(l.as_str()).unwrap())
        .sorted()
        .collect();

    println!(
        "{}",
        h.iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.bid)
            .sum::<u64>()
    );

    let hand_part2: Vec<_> = h
        .iter()
        .map(|h| {
            let tmp2: [Card; 5] = h
                .cards
                .iter()
                .map(|c| match c {
                    Card::Jack => Card::Number(1),
                    c => c.clone(), // TODO: convince the compiler that move would be fine?
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            Hand {
                cards: tmp2,
                bid: h.bid,
            }
        })
        .sorted()
        .collect();

    println!(
        "{}",
        hand_part2
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.bid)
            .sum::<u64>()
    );
}
