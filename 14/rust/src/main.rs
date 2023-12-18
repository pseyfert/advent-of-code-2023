// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Cube,
    Round,
    Empty,
}

fn is_barrier(s: Space) -> bool {
    match s {
        Space::Cube => true,
        _ => false,
    }
}

impl TryFrom<char> for Space {
    type Error = ();
    fn try_from(c: char) -> Result<Space, ()> {
        match c {
            '.' => Ok(Space::Empty),
            '#' => Ok(Space::Cube),
            'O' => Ok(Space::Round),
            _ => Err(()),
        }
    }
}

// fn compute_single_group<I: Iterator<Item = (Space, i64)> + DoubleEndedIterator>(iter: I) -> i64 {
fn compute_single_group<I: Iterator<Item = (Space, i64)>>(iter: I) -> i64 {
    let tmp = iter.collect::<Vec<_>>();
    let round_rocks_here = tmp.iter().filter(|(s, _)| *s == Space::Round).count();
    tmp.iter()
        .rev()
        .take(round_rocks_here)
        .map(|(_, v)| v)
        .sum::<i64>()
}

fn compute_column<I: Iterator<Item = char> + DoubleEndedIterator>(iter: I) -> i64 {
    iter.rev()
        .enumerate()
        .map(|(i, c)| -> (Space, i64) { (c.try_into().unwrap(), (i + 1).try_into().unwrap()) })
        .group_by(|(c, _)| is_barrier(c.clone()))
        .into_iter()
        .filter_map(|(k, g)| if k { None } else { Some(g.into_iter()) })
        .map(compute_single_group)
        .sum::<i64>()
}

use std::{collections::HashSet, io::BufRead, str::FromStr};

fn main() {
    let input_file = just_a_filename::Cli::parse().path;

    let space_map: [[char; 100]; 100] =
        std::io::BufReader::new(std::fs::File::open(input_file.clone()).unwrap())
            .lines()
            .map(|l| -> [char; 100] {
                l.unwrap()
                    .chars()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

    let p1 = (0..100)
        .map(|col_id| -> i64 {
            let column = (0..100)
                .map(|row_id| space_map[row_id][col_id])
                .collect::<Vec<_>>();
            compute_column(column.into_iter())
        })
        .sum::<i64>();
    println!("{p1}");
}
