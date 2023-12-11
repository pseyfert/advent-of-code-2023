// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;
use rayon::prelude::*;
use thiserror::Error;

use std::{io::BufRead, str::FromStr};

fn main() {
    let input_file = just_a_filename::Cli::parse().path;

    let mut galaxies: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();

    std::io::BufReader::new(std::fs::File::open(input_file.clone()).unwrap())
        .lines()
        .enumerate()
        .for_each(|(row_id, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .for_each(|(col_id, c)| match c {
                    '.' => {}
                    '#' => {
                        galaxies.insert((row_id, col_id));
                        ()
                    }
                    _ => panic!("unexpected input"),
                })
        });
    let right_most = galaxies.iter().map(|(_, c)| c).max().unwrap();
    let lowest = galaxies.iter().map(|(r, _)| r).max().unwrap();

    let empty_columns: Vec<_> = (0usize..*right_most)
        .filter(|c_scan| galaxies.iter().all(|(_, c_g)| c_g != c_scan))
        .collect();
    let empty_rows: Vec<_> = (0usize..*lowest)
        .filter(|r_scan| galaxies.iter().all(|(r_g, _)| r_g != r_scan))
        .collect();

    println!("empty columns: {empty_columns:?}");
    println!("empty rows: {empty_rows:?}");

    let quick_test = galaxies.iter().combinations(2).count();
    println!("need to do {quick_test} pairs");
}
