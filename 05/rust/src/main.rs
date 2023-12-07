// cSpell:words alman
use just_a_filename::prelude::*;
use rayon::prelude::*;

use std::io::BufRead;

#[derive(Debug)]
struct Row {
    in_start: i64,
    offset: i64,
    len: i64,
}

#[derive(Debug)]
struct Table(Vec<Row>);

fn advance_interval_stage(input: Intervals, alman: &Table) -> Intervals {
    let mut in_iter = input.0.iter();
    // let mut tab_iter = alman.0.iter();

    // let Some(mut current_table_row) = tab_iter.next() else {
    //     panic!("empty table?");
    // };

    let mut retval = Vec::new();

    while let Some(range) = in_iter.next() {
        let (ra, re) = range.clone().into_inner();
        match lb(ra, &alman) {
            None => {
                // current interval starts before the table
                if &re < &alman.0.get(0).unwrap().in_start {
                    // entire current interval before the table
                    retval.push(range.clone());
                } else {
                    retval.push(ra..=alman.0.get(0).unwrap().in_start - 1);
                    // continue with (alman.0.get(0).unwrap().in_start..=re)
                }
            }
            Some(i) => {
                // current interval after i.start
                let left = alman.0.get(i).unwrap();
                if ra > left.in_start + left.len {
                    // TODO: check i+1 is still in range
                    if &re < &alman.0.get(i + 1).unwrap().in_start {
                        retval.push(range.clone());
                    } else {
                        retval.push(ra..=alman.0.get(i + 1).unwrap().in_start - 1);
                        // continue with (alman.0.get(i+1).unwrap().in_start..=re)
                    }
                } else {
                    todo!()
                }
            }
        }
    }
    todo!();

    // let Included(start_range) = input.start_bound() else {
    //     panic!("???");
    // };
    // let lower_bound = match alman.0.binary_search_by(|row| row.in_start.cmp(&start_input)) {
    //     Ok(i) => i,
    //     Err(i) => {
    //         if i == 0 {
    //             return input;
    //         } else {
    //             i - 1
    //         }
    //     }
    // };
    // let entry = alman.0.get(lower_bound).unwrap();
    // input
    //     + if (input - entry.in_start) < entry.len {
    //         entry.offset
    //     } else {
    //         0
    //     }
}

fn lb(sought: i64, alman: &Table) -> Option<usize> {
    match alman.0.binary_search_by(|row| row.in_start.cmp(&sought)) {
        Ok(i) => Some(i),
        Err(i) => {
            if i == 0 {
                None
            } else {
                Some(i - 1)
            }
        }
    }
}

fn advance_stage(input: i64, alman: &Table) -> i64 {
    let Some(lower_bound) = lb(input, alman) else {
        return input;
    };
    let entry = alman.0.get(lower_bound).unwrap();
    input
        + if (input - entry.in_start) < entry.len {
            entry.offset
        } else {
            0
        }
}

fn all_stages(input: i64, almanac: &Vec<Table>) -> i64 {
    almanac
        .iter()
        .fold(input, |x, table| advance_stage(x, table))
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5, Table(vec![]), 5)]
    #[case(5, Table(vec![Row{in_start: 0, offset: 0, len: 1}]), 5)]
    #[case(5, Table(vec![Row{in_start: 5, offset: 0, len: 1}]), 5)]
    #[case(5, Table(vec![Row{in_start: 5, offset: 0, len: 9}]), 5)]
    #[case(5, Table(vec![Row{in_start: 3, offset: 0, len: 9}]), 5)]
    #[case(5, Table(vec![Row{in_start: 3, offset: 0, len: 3}]), 5)]
    #[case(5, Table(vec![Row{in_start: 3, offset: 0, len: 2}]), 5)]
    #[case(5, Table(vec![Row{in_start: 1, offset: 0, len: 3}]), 5)]
    #[case(5, Table(vec![Row{in_start: 8, offset: 0, len: 3}]), 5)]
    #[case(5, Table(vec![Row{in_start: 0, offset: 10, len: 1}]), 5)]
    #[case(5, Table(vec![Row{in_start: 5, offset: 10, len: 1}]), 15)]
    #[case(5, Table(vec![Row{in_start: 5, offset: 10, len: 9}]), 15)]
    #[case(5, Table(vec![Row{in_start: 3, offset: 10, len: 9}]), 15)]
    #[case(5, Table(vec![Row{in_start: 3, offset: 10, len: 3}]), 15)]
    #[case(5, Table(vec![Row{in_start: 3, offset: 10, len: 2}]), 5)]
    #[case(5, Table(vec![Row{in_start: 1, offset: 10, len: 3}]), 5)]
    #[case(5, Table(vec![Row{in_start: 8, offset: 10, len: 3}]), 5)]
    fn test_advancement(#[case] input: i64, #[case] alman: Table, #[case] reference: i64) {
        assert_eq!(advance_stage(input, &alman), reference);
    }

    #[test]
    fn test_stages() {
        let all = vec![
            Table(vec![Row {
                in_start: 4,
                offset: 1,
                len: 3,
            }]),
            Table(vec![Row {
                in_start: 4,
                offset: 1,
                len: 3,
            }]),
            Table(vec![Row {
                in_start: 4,
                offset: 1,
                len: 3,
            }]),
            Table(vec![Row {
                in_start: 4,
                offset: 1,
                len: 3,
            }]),
            Table(vec![Row {
                in_start: 4,
                offset: 1,
                len: 3,
            }]),
        ];

        assert_eq!(all_stages(4, &all), 7);
    }

    #[test]
    fn test_intervals() {
        assert_eq!(Intervals(vec![(1..=5)]), merge((1..=4), (2..=5)));
        assert_eq!(Intervals(vec![(1..=10)]), merge((1..=10), (2..=5)));
        assert_eq!(Intervals(vec![(1..=3), (9..=10)]), merge((1..=3), (9..=10)));
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Intervals(Vec<std::ops::RangeInclusive<i64>>);

fn merge(a: std::ops::RangeInclusive<i64>, b: std::ops::RangeInclusive<i64>) -> Intervals {
    let (aa, ae) = a.clone().into_inner();
    let (ba, be) = b.clone().into_inner();
    assert!(aa < ba);
    if ba <= ae {
        Intervals(vec![(aa..=std::cmp::max(be, ae))])
    } else {
        Intervals(vec![a, b])
    }
}

fn main() {
    let mut line_iter =
        std::io::BufReader::new(std::fs::File::open(just_a_filename::Cli::parse().path).unwrap())
            .lines();

    let seeds: Vec<_> = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    line_iter.next();
    line_iter.next();

    let mut cur_table = Table(vec![]);
    let mut table_vec = vec![];
    while let Some(Ok(cur_line)) = line_iter.next() {
        if cur_line == "" {
            // cur_table.0.sort_by(|a, b| a.in_start.cmp(&b.in_start));
            cur_table
                .0
                .par_sort_unstable_by(|a, b| a.in_start.cmp(&b.in_start));
            table_vec.push(cur_table);
            cur_table = Table(vec![]);
            line_iter.next();
            continue;
        }
        let mut split_iter = cur_line.split(' '); //.iter();
        let (Some(out_start), Some(in_start), Some(len), None) = (
            split_iter.next(),
            split_iter.next(),
            split_iter.next(),
            split_iter.next(),
        ) else {
            println!("parsing error: {cur_line}");
            return ();
        };
        let (in_start, out_start, len) = (
            in_start.parse::<i64>().unwrap(),
            out_start.parse::<i64>().unwrap(),
            len.parse::<i64>().unwrap(),
        );
        cur_table.0.push(Row {
            in_start,
            offset: out_start - in_start,
            len,
        });
    }
    cur_table
        .0
        .par_sort_unstable_by(|a, b| a.in_start.cmp(&b.in_start));
    table_vec.push(cur_table);

    let part1 = seeds
        .par_iter()
        .map(|s| all_stages(*s, &table_vec))
        // .map(|s| advance_stage(*s, &table_vec[0]))
        // .map(|s| advance_stage(s, &table_vec[1]))
        // .collect();
        .min()
        .unwrap();

    println!("{part1:?}");
}
