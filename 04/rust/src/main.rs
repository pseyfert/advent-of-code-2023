extern crate unicode_segmentation;

use serde::{de, Deserialize};
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct Card {
    pub id: u32,
    pub winning: Vec<u32>,
    pub having: Vec<u32>,
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Card, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v: &str = de::Deserialize::deserialize(deserializer)?;
        let mut split_1 = v.split(':');
        let (Some(card_identifier), Some(numbers), None) =
            (split_1.next(), split_1.next(), split_1.next())
        else {
            return Err(de::Error::custom("parsing error"));
        };

        let mut split_2 = numbers.split('|');
        let (Some(winning_numbers), Some(having_numbers), None) =
            (split_2.next(), split_2.next(), split_2.next())
        else {
            return Err(de::Error::custom("parsing error"));
        };

        let winning_numbers = winning_numbers.trim();
        let having_numbers = having_numbers.trim();

        let mut wn: Vec<_> = winning_numbers
            .split(' ')
            .filter_map(|n| if n == "" { None } else { n.parse::<u32>().ok() })
            .collect();
        let mut hn: Vec<_> = having_numbers
            .split(' ')
            .filter_map(|n| if n == "" { None } else { n.parse::<u32>().ok() })
            .collect();
        wn.sort();
        hn.sort();

        Ok(Card {
            id: 0,
            winning: wn,
            having: hn,
        })
    }
}

fn compute_matches(c: &Card) -> u32 {
    let mut w = c.winning.iter();
    let mut h = c.having.iter();

    let mut wo = w.next();
    let mut ho = h.next();

    let mut matches = 0;

    while let (Some(wv), Some(hv)) = (wo, ho) {
        match wv.cmp(hv) {
            std::cmp::Ordering::Equal => {
                matches += 1;
                (wo, ho) = (w.next(), h.next());
            }
            std::cmp::Ordering::Less => {
                wo = w.next();
            }
            std::cmp::Ordering::Greater => {
                ho = h.next();
            }
        }
    }
    matches
}

fn compute_score(matches: u32) -> u32 {
    let mut score = 0;

    for _ in 0..matches {
        if score == 0 {
            score = 1;
        } else {
            score = score * 2;
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}

fn main() {
    // TODO: a serde "every line is an element" would be nicer
    let input: Vec<Card> = std::io::BufReader::new(std::fs::File::open("../input").unwrap())
        .lines()
        .map(|l| serde_plain::from_str(&l.unwrap()).unwrap())
        .collect();
    println!("{:?}", input);
    println!(
        "score in part 1: {}",
        input.iter().map(|c| compute_score(compute_matches(c))).sum::<u32>()
    );
}
