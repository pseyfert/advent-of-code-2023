// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;
use rayon::prelude::*;
use thiserror::Error;

use std::collections::HashMap;

use std::{io::BufRead, str::FromStr};

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

mod part2;
use part2::{CycleDetector, Cycles, InstState};

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

fn p2_brute(
    instructions: &Vec<Instruction>,
    state: (Vec<[char; 3]>, usize),
    left_map: &HashMap<[char; 3], [char; 3]>,
    right_map: &HashMap<[char; 3], [char; 3]>,
) {
    let part2 = instructions
        .iter()
        .cycle()
        .scan(state, |pos, i| {
            if (pos.0).par_iter().all(|loc| loc[2] == 'Z') {
                None
            } else {
                pos.0.par_iter_mut().for_each(|loc| {
                    *loc = match i {
                        Instruction::Left => left_map[loc],
                        Instruction::Right => right_map[loc],
                    };
                });
                // println!("debug step {}: {:?}", pos.1, pos.0);
                pos.1 += 1;
                Some(pos.1)
            }
        })
        .count();

    println!("{part2}");
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

    let p2_starts: Vec<_> = left_map.keys().cloned().filter(|l| l[2] == 'A').collect();

    let state: (Vec<[char; 3]>, usize) = (p2_starts.clone(), 0);

    p2_starts
        .into_iter()
        .map(|start_loc| {
            instructions
                .iter()
                .enumerate()
                .cycle()
                .enumerate()
                .map(|(total_i, (inst_i, inst))| {
                    let run_i = total_i / instructions.len();
                    (inst, InstState((inst_i, run_i)))
                })
                .scan(
                    (start_loc, CycleDetector::new()),
                    |(pos, detector), inst| {
                        *pos = match inst.0 {
                            Instruction::Left => left_map[pos],
                            Instruction::Right => right_map[pos],
                        };

                        match detector.add_dest(*pos, inst.1) {
                            // Here I need to trick the `scan` abort condition
                            None => Some(None),
                            Some(result) => Some(Some(result)),
                        }
                    },
                )
                .find_map(|ooc| ooc)
                .expect("never went into cycle?")
        })
        .collect::<Vec<Cycles>>();
}
