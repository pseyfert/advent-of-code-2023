// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;

use std::{io::BufRead, str::FromStr};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
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
        let mut tmp: [Card; 5] = [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];
        for (i, c) in cards
            .chars()
            .map(|c| Card::try_from(c).unwrap())
            .enumerate()
        {
            tmp[i] = c;
        }
        Ok(Hand {
            cards: tmp,
            bid: bid.parse::<u64>().unwrap(),
        })
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Card) -> std::cmp::Ordering {
        match (self, other) {
            (Card::Number(l), Card::Number(r)) => l.cmp(r),
            (Card::Number(_), _) => std::cmp::Ordering::Less,
            (_, Card::Number(_)) => std::cmp::Ordering::Greater,
            (Card::Ace, Card::Ace) => std::cmp::Ordering::Equal,
            (Card::Ace, _) => std::cmp::Ordering::Greater,
            (_, Card::Ace) => std::cmp::Ordering::Less,
            (Card::King, Card::King) => std::cmp::Ordering::Equal,
            (Card::King, _) => std::cmp::Ordering::Greater,
            (_, Card::King) => std::cmp::Ordering::Less,
            (Card::Queen, Card::Queen) => std::cmp::Ordering::Equal,
            (Card::Queen, _) => std::cmp::Ordering::Greater,
            (_, Card::Queen) => std::cmp::Ordering::Less,
            (Card::Jack, Card::Jack) => std::cmp::Ordering::Equal,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Value {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

impl Ord for Value {
    fn cmp(&self, other: &Value) -> std::cmp::Ordering {
        if self == other {
            std::cmp::Ordering::Equal
        } else {
            match (self, other) {
                (Value::FiveOfAKind, _) => std::cmp::Ordering::Greater,
                (_, Value::FiveOfAKind) => std::cmp::Ordering::Less,
                (Value::FourOfAKind, _) => std::cmp::Ordering::Greater,
                (_, Value::FourOfAKind) => std::cmp::Ordering::Less,
                (Value::FullHouse, _) => std::cmp::Ordering::Greater,
                (_, Value::FullHouse) => std::cmp::Ordering::Less,
                (Value::ThreeOfAKind, _) => std::cmp::Ordering::Greater,
                (_, Value::ThreeOfAKind) => std::cmp::Ordering::Less,
                (Value::TwoPairs, _) => std::cmp::Ordering::Greater,
                (_, Value::TwoPairs) => std::cmp::Ordering::Less,
                (Value::OnePair, _) => std::cmp::Ordering::Greater,
                (_, Value::OnePair) => std::cmp::Ordering::Less,
                (Value::HighCard, Value::HighCard) => panic!("unreachable"),
            }
        }
    }
}
impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Value) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn value(h: &Hand) -> Value {
    let mut tmp: Vec<_> = h.cards.iter().collect();
    tmp.sort();
    let mut grml = Vec::new();
    for (_k, g) in &tmp.iter().group_by(|c| c.clone()) {
        let mut why = 0;
        for _gg in g {
            why += 1;
        }

        grml.push(why);
    }
    grml.sort();
    match grml.len() {
        1 => Value::FiveOfAKind,
        2 => {
            if *grml.get(0).unwrap() == 1 {
                Value::FourOfAKind
            } else {
                Value::FullHouse
            }
        }
        3 => {
            if *grml.get(2).unwrap() == 3 {
                Value::ThreeOfAKind
            } else {
                Value::TwoPairs
            }
        }
        4 => Value::OnePair,
        5 => Value::HighCard,
        _ => panic!("more than 5 cards?"),
    }
}

fn value_joker(h: &Hand) -> Value {
    let mut tmp: Vec<_> = h
        .cards
        .iter()
        .filter_map(|c| match c {
            Card::Number(1) => None,
            c => Some(c.clone()),
        })
        .collect();
    tmp.sort();
    let mut grml = Vec::new();
    for (_k, g) in &tmp.iter().group_by(|c| c.clone()) {
        let mut why = 0;
        for _gg in g {
            why += 1;
        }

        grml.push(why);
    }
    grml.sort();
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
fn hand_ord_joker(lhs: &Hand, rhs: &Hand) -> std::cmp::Ordering {
    match value_joker(lhs).cmp(&value_joker(rhs)) {
        std::cmp::Ordering::Equal => {
            for (lc, rc) in lhs.cards.iter().zip(&rhs.cards) {
                if *lc != *rc {
                    return lc.cmp(rc);
                }
            }
            std::cmp::Ordering::Equal
        }
        o => o,
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        match value(self).cmp(&value(other)) {
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
    let mut h: Vec<_> = line_iter
        .map(|l| Hand::from_str(l.as_str()).unwrap())
        .collect();
    h.sort();
    // println!("{h:?}");

    println!(
        "{}",
        h.iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.bid)
            .sum::<u64>()
    );

    let mut hand_part2: Vec<_> = h
        .iter()
        .map(|h| {
            let mut tmp2: [Card; 5] = [Card::Ace, Card::Ace, Card::Ace, Card::Ace, Card::Ace];
            for (i, c) in h
                .cards
                .iter()
                .map(|c| match c {
                    Card::Jack => Card::Number(1),
                    c => c.clone(), // TODO: convince the compiler that move would be fine?
                })
                .enumerate()
            {
                tmp2[i] = c;
            }

            Hand {
                cards: tmp2,
                bid: h.bid,
            }
        })
        .collect();
    hand_part2.sort_by(hand_ord_joker);

    println!(
        "{}",
        hand_part2
            .iter()
            .enumerate()
            .map(|(i, h)| (i + 1) as u64 * h.bid)
            .sum::<u64>()
    );
}
