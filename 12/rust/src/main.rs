// cSpell:words alman
use is_sorted::IsSorted;
use itertools::Itertools;
use just_a_filename::prelude::*;
use rayon::prelude::*;
use thiserror::Error;

use std::{collections::HashSet, io::BufRead, str::FromStr};

#[derive(Clone, PartialEq, Eq, Debug)]
enum Spring {
    Working,
    Broken,
    Unknown,
}

#[derive(Debug)]
struct CheckSum(Vec<usize>);

#[derive(Debug)]
struct Row(Vec<Spring>);

fn ensure_no_overlaps_and_in_row(row: &Row, check_sum: &CheckSum, combination: &[usize]) -> bool {
    assert!(combination.iter().is_sorted());
    assert_eq!(check_sum.0.len(), combination.len());

    if check_sum
        .0
        .iter()
        .zip(combination)
        .map(|(len, index)| len + index > row.0.len())
        // .inspect(|b| {
        //     print!("{}", b);
        // })
        .last()
        .unwrap()
    {
        // println!("{combination:?} {check_sum:?} too var right in {row:?}");
        return false;
    }
    // println!("{combination:?} {check_sum:?} left enough for {row:?}");
    // println!("pass");
    if check_sum
        .0
        .iter()
        .zip(combination)
        .collect::<Vec<_>>()
        .windows(2)
        .find(|i| {
            let mut it = i.into_iter();
            let (Some(lhs), Some(rhs), None) = (it.next(), it.next(), it.next()) else {
                panic!("logic error");
            };
            if lhs.0 + lhs.1 >= *rhs.1 {
                // println!("{combination:?} with {check_sum:?} has overlap at {lhs:?} with {rhs:?}");
                true
            } else {
                // println!("no overlap {combination:?} with {check_sum:?} at {lhs:?} with {rhs:?}");
                false
            }
        })
        .is_some()
    {
        // println!("combination w/o any overlap");
        return false;
    }
    return true;
}

fn combination_to_row(len: usize, check_sum: &CheckSum, combination: &[usize]) -> Row {
    let mut tmp = vec![Spring::Working; len];
    for (index, len) in combination.iter().zip(check_sum.0.iter()) {
        for i in *index..(*index + len) {
            tmp[i] = Spring::Broken;
        }
    }

    let rv = Row(tmp.to_vec());
    // println!("{rv:?}");
    rv
}

fn validate_combination(row: &Row, check_sum: &CheckSum, combination: &[usize]) -> bool {
    // combination is not a particularly smart representation of "where could the broken blocks
    // be".
    // With all the non-overlapping and one gap, the problem is far more constrained than the
    // N(groups) possible starting positions, but i'm too lazy to come up with a good generation of
    // allowed constellations.
    //
    // That said, the program is way too slow for part 2. I'm not convinced that the issue I'm
    // discussing here would resolve that. After all, I'm still ignoring the entire constraint from
    // the Row. … and in part 2 also the periodic structure.
    if !ensure_no_overlaps_and_in_row(row, check_sum, combination) {
        return false;
    }

    let hypothesis = combination_to_row(row.0.len(), check_sum, combination);

    let passes = row.0.iter().zip(hypothesis.0.iter()).all(|(known, hypo)| {
        if *known == Spring::Unknown {
            true
        } else if *known == *hypo {
            true
        } else {
            false
        }
    });

    // println!("{row:?}\n{hypothesis:?}\n => {passes}\n\n");
    passes
}

fn main() {
    let input_file = just_a_filename::Cli::parse().path;

    let row_and_checksum: Vec<_> =
        std::io::BufReader::new(std::fs::File::open(input_file.clone()).unwrap())
            .lines()
            .map(|l| {
                let l = l.unwrap();
                let mut it = l.split(' ');
                let (Some(record), Some(check), None) = (it.next(), it.next(), it.next()) else {
                    panic!("...");
                };
                let mut row: Row = Row(Vec::new());
                for c in record.chars() {
                    match c {
                        '.' => row.0.push(Spring::Working),
                        '#' => row.0.push(Spring::Broken),
                        '?' => row.0.push(Spring::Unknown),
                        _ => panic!("unexpected character"),
                    }
                }
                (
                    row,
                    CheckSum(
                        check
                            .split(',')
                            .map(|num| num.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    ),
                )
            })
            .collect();

    let p1 = row_and_checksum
        .par_iter()
        .map(|(row, check_sum)| {
            (0..row.0.len())
                .combinations(check_sum.0.len())
                .map(|combination| {
                    if validate_combination(row, check_sum, &combination) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i64>()
        })
        // .collect::<Vec<_>>();
        .sum::<i64>();
    println!("{p1:?}");

    let p2 = row_and_checksum
        .par_iter()
        .map(|(row, check_sum)| {
            let check_sum = CheckSum(
                check_sum
                    .0
                    .iter()
                    .chain(check_sum.0.iter())
                    .chain(check_sum.0.iter())
                    .chain(check_sum.0.iter())
                    .chain(check_sum.0.iter())
                    .cloned()
                    .collect::<Vec<_>>(),
            );

            let row = Row(
                row.0.iter()
                    .chain(std::iter::once(&Spring::Unknown))
                    .chain(row.0.iter())
                    .chain(std::iter::once(&Spring::Unknown))
                    .chain(row.0.iter())
                    .chain(std::iter::once(&Spring::Unknown))
                    .chain(row.0.iter())
                    .chain(std::iter::once(&Spring::Unknown))
                    .chain(row.0.iter())
                    .cloned()
                    .collect::<Vec<_>>(),
            );


            (0..row.0.len())
                .combinations(check_sum.0.len())
                .map(|combination| {
                    if validate_combination(&row, &check_sum, &combination) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i64>()
        })
        // .collect::<Vec<_>>();
        .sum::<i64>();
    println!("{p1:?}");
}
