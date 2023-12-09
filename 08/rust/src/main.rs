// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;
use thiserror::Error;

use std::{io::BufRead, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Error, Debug)]
#[error("parser error")]
struct InvalidCharForDirection {}

impl TryFrom<char> for Instruction {
    type Error = InvalidCharForDirection;
    fn try_from(c: char) -> Result<Instruction, InvalidCharForDirection> {
        match c {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(InvalidCharForDirection {}),
        }
    }
}

fn main() {
    let mut line_iter =
        std::io::BufReader::new(std::fs::File::open(just_a_filename::Cli::parse().path).unwrap())
            .lines()
            .map(|l| l.unwrap());

    let instructions: Vec<Instruction> = line_iter
        .next()
        .unwrap()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect();

    line_iter.next();

    let r = regex::Regex::new(
        "^(?<from>[A-Z][A-Z][A-Z]) = \\((?<left>[A-Z][A-Z][A-Z]), (?<right>[A-Z][A-Z][A-Z])\\)$",
    )
    .unwrap();

    let mut left_map: std::collections::HashMap<[char; 3], [char; 3]> =
        std::collections::HashMap::new();
    let mut right_map: std::collections::HashMap<[char; 3], [char; 3]> =
        std::collections::HashMap::new();

    line_iter.for_each(|l| {
        let Some(cap) = r.captures(&l) else {
            panic!("couldn't match {l}");
        };

        let f: [char; 3] = cap
            .name("from")
            .unwrap()
            .as_str()
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let l: [char; 3] = cap
            .name("left")
            .unwrap()
            .as_str()
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let r: [char; 3] = cap
            .name("right")
            .unwrap()
            .as_str()
            .chars()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        left_map.insert(f, l);
        right_map.insert(f, r);
    });

    // todo!();

    let state: ([char; 3], usize) = (['A', 'A', 'A'], 0);

    let part1 = instructions
        .iter()
        .cycle()
        .scan(state, |pos, i| {
            if (pos.0).eq(&['Z', 'Z', 'Z']) {
                None
            } else {
                *pos = match i {
                    Instruction::Left => (left_map[&pos.0], pos.1 + 1),
                    Instruction::Right => (right_map[&pos.0], pos.1 + 1),
                };
                // println!("debug step {}: {:?}", pos.1, pos.0);
                Some(pos.1)
            }
        })
        .count();
    println!("{part1}");
}
