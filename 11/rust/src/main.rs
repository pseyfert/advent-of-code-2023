// cSpell:words alman
use itertools::Itertools;
use just_a_filename::prelude::*;
use rayon::prelude::*;
use thiserror::Error;

use std::{collections::HashSet, io::BufRead, str::FromStr};

fn galaxy_distances(
    galaxies: &HashSet<(usize, usize)>,
    empty_rows: &[usize],
    empty_columns: &[usize],
    expansion: i64
) -> i64 {
    galaxies
        .iter()
        .combinations(2)
        .map(|comb| {
            let mut it = comb.iter();
            let (Some(a), Some(b), None) = (it.next(), it.next(), it.next()) else {
                panic!("combinations are strange");
            };
            // TODO: std::cmp::minmax
            let (high, low) = (std::cmp::min(a.0, b.0), std::cmp::max(a.0, b.0));
            let (left, right) = (std::cmp::min(a.1, b.1), std::cmp::max(a.1, b.1));
            (high..low)
                .map(|r| match empty_rows.binary_search(&r) {
                    Ok(_) => expansion,
                    Err(_) => 1,
                })
                .sum::<i64>()
                + (left..right)
                    .map(|c| match empty_columns.binary_search(&c) {
                        Ok(_) => expansion,
                        Err(_) => 1,
                    })
                    .sum::<i64>()
        })
        .sum::<i64>()
}

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

    let p1 = galaxy_distances(&galaxies, &empty_rows, &empty_columns, 2);
    let p2_ex = galaxy_distances(&galaxies, &empty_rows, &empty_columns, 10);
    let p2 = galaxy_distances(&galaxies, &empty_rows, &empty_columns, 1000000);
    println!("{p1}");
    println!("{p2_ex}");
    println!("{p2}");
}
