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

    let res = input
        .iter()
        .map(|measurement_series| {
            (0..)
                .scan(measurement_series.clone(), |state, _| {
                    // return the last element of the current row and update state to the adjacent
                    // difference.
                    let rv = (*state.first().unwrap(), *state.last().unwrap());

                    let rv = if state.par_iter().all(|v| *v == 0) {
                        None
                    } else {
                        Some(rv)
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
                .map(|(i, (f, l))| if i % 2 == 0 { (f, l) } else { (-f, l) })
                .fold((0, 0), |(af, al), (f, l)| (af + f, al + l))
        })
        .fold((0, 0), |(af, al), (f, l)| (af + f, al + l));
    println!("Part 1: {}", res.1);
    println!("Part 2: {}", res.0);
}
