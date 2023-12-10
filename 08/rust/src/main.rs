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
    let i_len: u128 = instructions.len() as u128;

    let p2_cycle_analysis = p2_starts
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

                        if pos[2] == 'Z' {
                            match detector.add_dest(*pos, inst.1) {
                                // Here I need to trick the `scan` abort condition
                                None => Some(None),
                                Some(result) => Some(Some(result)),
                            }
                        } else {
                            Some(None)
                        }
                    },
                )
                .find_map(|ooc| ooc)
                .expect("never went into cycle?")
        })
        .collect::<Vec<Cycles>>();

    p2_cycle_analysis.iter().for_each(|c| {
        println!("done here: {c:?}");
    });
    // turns out:
    // no end zone is reached before entering a cycle. (so much for my partition)
    // every cycle contains exactly one destination. (so much for all my vec)
    // the remaining logic below may or may not be correct, i obtained the result purely by looking
    // at the printout above.
    //
    let asdf: Vec<_> = p2_cycle_analysis
        .iter()
        .map(|i| {
            let inst_state = i.cycle_visits.get(0).unwrap();
            let first_encounter: u128 = i_len * TryInto::<u128>::try_into(inst_state.0 .1).unwrap()
                + TryInto::<u128>::try_into(inst_state.0 .0).unwrap();
            let cycle_len: u128 = i_len * TryInto::<u128>::try_into(i.cycle_len).unwrap();
            (first_encounter as u128, cycle_len as u128)
        })
        .collect();

    let latest_arriver = asdf.iter().max_by(|l, r| l.0.cmp(&r.0)).unwrap();
    let full_cycle = i_len * asdf.iter().map(|p| p.1).product::<u128>();
    let fuse: u128 = latest_arriver.1 * i_len + latest_arriver.0 + full_cycle;

    let p2 = (0..)
        .scan(asdf, |state, _| {
            // println!("state now is {state:?}");
            let first = state.first().unwrap();
            if state.iter().skip(1).all(|i| i.0 == first.0) {
                None
            } else {
                let mut lowest: &mut (u128, u128) =
                    state.iter_mut().min_by(|l, r| l.0.cmp(&r.0)).unwrap();
                lowest.0 += lowest.1;
                assert!(lowest.0 < fuse);
                Some(state.clone())
            }
        })
        .last();
    println!("{p2:?}");
}
