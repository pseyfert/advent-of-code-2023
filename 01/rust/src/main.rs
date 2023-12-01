extern crate unicode_segmentation;

use serde::{de, Deserialize};
use std::io::BufRead;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
struct FancyInputEntry {
    pub inner: u32,
}

#[derive(Debug)]
struct InputEntry {
    pub inner: u32,
}

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn digit_words() -> std::collections::HashMap<&'static str, u32> {
    std::collections::HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ])
}

// TODO: Would be nice to deduplicate 'start' and 'end' functions.
fn start_with_real_digit(s: &str) -> Option<u32> {
    let found = DIGITS.iter().find(|&&ch| s.starts_with(ch));
    found.and_then(|c| c.to_string().parse::<u32>().ok())
}

fn ends_with_real_digit(s: &str) -> Option<u32> {
    let found = DIGITS.iter().find(|&&ch| s.ends_with(ch));
    found.and_then(|c| c.to_string().parse::<u32>().ok())
}

fn start_with_any_digit(s: &str) -> Option<u32> {
    let found = DIGITS.iter().find(|&&ch| s.starts_with(ch));
    found
        .and_then(|c| c.to_string().parse::<u32>().ok())
        .or_else(|| {
            digit_words().iter().find_map(|(&string, val)| {
                if s.starts_with(string) {
                    Some(*val)
                } else {
                    None
                }
            })
        })
}

fn ends_with_any_digit(s: &str) -> Option<u32> {
    let found = DIGITS.iter().find(|&&ch| s.ends_with(ch));
    found
        .and_then(|c| c.to_string().parse::<u32>().ok())
        .or_else(|| {
            digit_words().iter().find_map(|(&string, val)| {
                if s.ends_with(&string) {
                    Some(*val)
                } else {
                    None
                }
            })
        })
}

impl<'de> Deserialize<'de> for FancyInputEntry {
    fn deserialize<D>(deserializer: D) -> Result<FancyInputEntry, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v: &str = de::Deserialize::deserialize(deserializer)?;
        let starting_digit = start_graphemes(v).find_map(start_with_any_digit);
        let end_digit = end_graphemes(v).find_map(ends_with_any_digit);

        match (starting_digit, end_digit) {
            (Some(s), Some(e)) => Ok(FancyInputEntry { inner: 10 * s + e }),
            _ => Err(de::Error::custom("parsing error")),
        }
    }
}

impl<'de> Deserialize<'de> for InputEntry {
    fn deserialize<D>(deserializer: D) -> Result<InputEntry, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let v: &str = de::Deserialize::deserialize(deserializer)?;
        let starting_digit = start_graphemes(v).find_map(start_with_real_digit);
        let end_digit = end_graphemes(v).find_map(ends_with_real_digit);

        match (starting_digit, end_digit) {
            (Some(s), Some(e)) => Ok(InputEntry { inner: 10 * s + e }),
            _ => Err(de::Error::custom("parsing error")),
        }
    }
}

fn start_graphemes(line: &str) -> impl Iterator<Item = &str> {
    line.grapheme_indices(true).map(move |(i, _)| &line[i..])
}

fn end_graphemes(line: &str) -> impl Iterator<Item = &str> {
    line.grapheme_indices(true)
        .chain([(line.len(), "")])
        .rev()
        .map(move |(i, _)| &line[..i])
}

fn main() {
    // TODO: a serde "every line is an element" would be nicer
    let input: Vec<InputEntry> = std::io::BufReader::new(std::fs::File::open("../input").unwrap())
        .lines()
        .map(|l| serde_plain::from_str(&l.unwrap()).unwrap())
        .collect();
    println!("{}", input.iter().map(|e| e.inner).sum::<u32>());

    let input: Vec<FancyInputEntry> = std::io::BufReader::new(std::fs::File::open("../input").unwrap())
        .lines()
        .map(|l| serde_plain::from_str(&l.unwrap()).unwrap())
        .collect();
    println!("{}", input.iter().map(|e| e.inner).sum::<u32>());
}
