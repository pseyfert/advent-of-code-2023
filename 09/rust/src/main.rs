// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;
use rayon::prelude::*;
use thiserror::Error;

use std::collections::HashMap;

use std::{io::BufRead, str::FromStr};

fn main() {
    let input_file = just_a_filename::Cli::parse().path;
    let input: Vec<Vec<i64>> =
        std::io::BufReader::new(std::fs::File::open(input_file.clone()).unwrap())
            .lines()
            .map(|l| {
                l.unwrap()
                    .split(' ')
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect();

    let p1 = input
        .par_iter()
        .map(|measurement_series| -> i64 {
            (0..)
                .scan(measurement_series.clone(), |state, _| {
                    // return the last element of the current row and update state to the adjacent
                    // difference.
                    let rv = state.last().unwrap();

                    let rv = if state.par_iter().all(|v| *v == 0) {
                        None
                    } else {
                        Some(*rv)
                    };

                    *state = state
                        .par_windows(2)
                        .map(|two| {
                            let mut it = two.iter();
                            let (Some(lhs), Some(rhs), None) = (it.next(), it.next(), it.next())
                            else {
                                panic!("wtf?");
                            };
                            rhs - lhs
                        })
                        .collect::<Vec<i64>>();
                    rv
                })
                .sum::<i64>()
        })
        .sum::<i64>();
    println!("Part 1: {p1}");

    let p2 = input
        .par_iter()
        .map(|measurement_series| -> i64 {
            (0..)
                .scan(measurement_series.clone(), |state, _| {
                    // return the last element of the current row and update state to the adjacent
                    // difference.
                    let rv = state.first().unwrap();

                    let rv = if state.par_iter().all(|v| *v == 0) {
                        None
                    } else {
                        Some(*rv)
                    };

                    *state = state
                        .par_windows(2)
                        .map(|two| {
                            let mut it = two.iter();
                            let (Some(lhs), Some(rhs), None) = (it.next(), it.next(), it.next())
                            else {
                                panic!("wtf?");
                            };
                            rhs - lhs
                        })
                        .collect::<Vec<i64>>();
                    rv
                })
                .enumerate()
                .map(|(i, c)| if i % 2 == 0 { c } else { -c })
                .sum::<i64>()
        })
        .sum::<i64>();
    println!("Part 2: {p2}");
}
