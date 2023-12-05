use itertools::Itertools;
use std::io::BufRead;

fn main() {
    let bare_input: Vec<_> = std::io::BufReader::new(std::fs::File::open("../input").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let input: Vec<_> = bare_input
        .iter()
        .map(|s| {
            let mut buffer = Vec::new();
            for (k, g) in &s.chars().enumerate().group_by(|(_, c)| c.is_ascii_digit()) {
                if k {
                    let mut it = g.into_iter();
                    let Some((start, _)) = it.next() else {
                        todo!();
                    };
                    let mut memory = start;
                    loop {
                        let Some((next, _)) = it.next() else {
                            break;
                        };
                        memory = next;
                    }
                    let end = memory + 1;
                    let num = s[start..end].parse::<u32>().unwrap();
                    buffer.push(((start, end), num));
                } else {
                }
            }
            buffer
        })
        .collect();

    let p1_almost = input
        .iter()
        .map(|v| v.iter().map(|(_, n)| n).sum::<u32>())
        .sum::<u32>()
        - 114
        - 58;

    let p1_really = input
        .iter()
        .enumerate()
        .map(|(row, pl)| {
            let above_opt = if row > 0 {
                bare_input.get(row - 1)
            } else {
                None
            };
            let current = bare_input.get(row).unwrap();
            let below_opt = bare_input.get(row + 1);

            pl.iter()
                .filter_map(|((start, end), num)| -> Option<u32> {
                    let start_m1 = if *start > 0 { start - 1 } else { 0 };
                    let end_p1 = if *end < current.len() { end + 1 } else { *end };
                    if (if *start > 0 {
                        current.chars().nth(start_m1)
                    } else {
                        None
                    })
                    .and_then(|c| if c == '.' { None } else { Some(c) })
                    .is_none()
                        && (if *end < current.len() {
                            current.chars().nth(*end)
                        } else {
                            None
                        })
                        .and_then(|c| if c == '.' { None } else { Some(c) })
                        .is_none()
                        && above_opt
                            .and_then(|line| {
                                if line[start_m1..end_p1].chars().all(|c| c == '.') {
                                    None
                                } else {
                                    Some(true)
                                }
                            })
                            .is_none()
                        && below_opt
                            .and_then(|line| {
                                if line[start_m1..end_p1].chars().all(|c| c == '.') {
                                    None
                                } else {
                                    Some(true)
                                }
                            })
                            .is_none()
                    {
                        None
                    } else {
                        Some(*num)
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();
    println!("{p1_almost}");
    println!("{p1_really}");
}
