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

fn advance_stage(input: i64, alman: &Table) -> i64 {
    let lower_bound = match alman.0.binary_search_by(|row| row.in_start.cmp(&input)) {
        Ok(i) => i,
        Err(i) => {
            if i == 0 {
                return input;
            } else {
                i - 1
            }
        }
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
