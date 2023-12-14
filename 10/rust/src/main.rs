use core::panic;
use just_a_filename::prelude::*;
use std::{io::BufRead, str::FromStr};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Cell {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Void,
    Start,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    N,
    S,
    E,
    W,
}

#[derive(Error, Debug)]
#[error("requested impossible state propagation")]
struct PropagationError {}
#[derive(Error, Debug)]
#[error("invalid character in map")]
struct ParsingError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row(usize);
#[derive(Debug, Clone, PartialEq, Eq)]
struct Col(usize);

#[derive(Debug, Clone)]
struct State {
    pos: (Row, Col),
    mom: Direction,
    steps: usize,
}

impl TryFrom<char> for Cell {
    type Error = ParsingError;
    fn try_from(c: char) -> Result<Cell, ParsingError> {
        match c {
            '|' => Ok(Cell::NS),
            '-' => Ok(Cell::EW),
            'L' => Ok(Cell::NE),
            'J' => Ok(Cell::NW),
            'F' => Ok(Cell::SE),
            '7' => Ok(Cell::SW),
            '.' => Ok(Cell::Void),
            'S' => Ok(Cell::Start),
            _ => Err(ParsingError {}),
        }
    }
}

#[derive(Error, Debug)]
enum PropException {
    #[error("out of map or wrong pipe")]
    ForbiddenPropagation(#[from] PropagationError),
    #[error("StartingTile")]
    Start(usize),
}

fn try_prop(input: &[[Cell; 140]; 140], s: &State, d: Direction) -> Result<State, PropException> {
    if (d == Direction::N && s.pos.0 .0 == 0) || (d == Direction::W && s.pos.1 .0 == 0) {
        return Err(PropagationError {}.into());
    }
    let pos = match d {
        Direction::N => (Row(s.pos.0 .0 - 1), s.pos.1.clone()),
        Direction::S => (Row(s.pos.0 .0 + 1), s.pos.1.clone()),
        Direction::E => (s.pos.0.clone(), Col(s.pos.1 .0 + 1)),
        Direction::W => (s.pos.0.clone(), Col(s.pos.1 .0 - 1)),
    };

    let cell = at(input, &pos)?;
    if cell == Cell::Start {
        return Err(PropException::Start(s.steps + 1));
    }

    let mom = match (d, cell) {
        (Direction::N, Cell::NS) => Direction::N,
        (Direction::N, Cell::SE) => Direction::E,
        (Direction::N, Cell::SW) => Direction::W,
        (Direction::N, _) => {
            return Err(PropagationError {}.into());
        }
        (Direction::S, Cell::NS) => Direction::S,
        (Direction::S, Cell::NE) => Direction::E,
        (Direction::S, Cell::NW) => Direction::W,
        (Direction::S, _) => {
            return Err(PropagationError {}.into());
        }
        (Direction::E, Cell::EW) => Direction::E,
        (Direction::E, Cell::NW) => Direction::N,
        (Direction::E, Cell::SW) => Direction::S,
        (Direction::E, _) => {
            return Err(PropagationError {}.into());
        }
        (Direction::W, Cell::EW) => Direction::W,
        (Direction::W, Cell::NE) => Direction::N,
        (Direction::W, Cell::SE) => Direction::S,
        (Direction::W, _) => {
            return Err(PropagationError {}.into());
        }
    };
    Ok(State {
        pos,
        mom,
        steps: s.steps + 1,
    })
}

fn at(input: &[[Cell; 140]; 140], s: &(Row, Col)) -> Result<Cell, PropagationError> {
    if s.0 .0 == input.len() || s.1 .0 == input[0].len() {
        Err(PropagationError {})
    } else {
        Ok(input[s.0 .0][s.1 .0].clone())
    }
}

fn main() {
    let input: [[Cell; 140]; 140] =
        std::io::BufReader::new(std::fs::File::open(just_a_filename::Cli::parse().path).unwrap())
            .lines()
            .map(|l| -> [Cell; 140] {
                l.unwrap()
                    .chars()
                    .into_iter()
                    .map(Cell::try_from)
                    .map(|c| c.unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

    let start = input
        .iter()
        .enumerate()
        .find_map(|(row_id, row)| {
            row.iter().enumerate().find_map(|(col_id, cell)| {
                if *cell == Cell::Start {
                    Some((Row(row_id), Col(col_id)))
                } else {
                    None
                }
            })
        })
        .unwrap();

    let random_init_state = State {
        pos: start.clone(),
        mom: Direction::N,
        steps: 0,
    };

    let mut init_tests = vec![
        try_prop(&input, &random_init_state, Direction::N),
        try_prop(&input, &random_init_state, Direction::S),
        try_prop(&input, &random_init_state, Direction::W),
        try_prop(&input, &random_init_state, Direction::E),
    ];

    let init_tests: Vec<_> = init_tests
        .iter_mut()
        .filter_map(|r| r.as_mut().ok())
        .collect();
    assert_eq!(init_tests.len(), 2);
    assert!(init_tests.iter().all(|s| s.steps == 1));

    let mut state_iter: State = (**init_tests.first().unwrap()).clone();

    let main_loop: Vec<_> = (0..)
        .scan(state_iter, |state, _| {
            let tmp = try_prop(&input, &state.clone(), state.mom.clone());
            match tmp {
                Ok(new_state) => {
                    let rv = Some(new_state.pos.clone());
                    *state = new_state;
                    rv
                }
                Err(PropException::Start(result)) => None,
                _ => {
                    panic!();
                }
            }
        })
        .chain(std::iter::once(start))
        .chain(std::iter::once((**init_tests.first().unwrap()).pos.clone()))
        .collect();

    let p1 = main_loop.len() / 2;
    println!("{p1}");

    let mut inside = 0usize;
    for row in 0..140 {
        for col in 0..140 {
            if main_loop
                .iter()
                .find(|pos| **pos == (Row(row), Col(col)))
                .is_some()
            {
                continue;
            }
            let row_scan = if row < 70 { (0..row) } else { (row + 1..140) };

            let mut complete_crossings = 0usize;
            let mut open_east = false;
            let mut open_west = false;
            for scan_point in row_scan {
                if main_loop
                    .iter()
                    .find(|pos| **pos == (Row(scan_point), Col(col)))
                    .is_none()
                {
                    continue;
                }
                match at(&input, &(Row(scan_point), Col(col))).unwrap() {
                    Cell::EW => complete_crossings += 1,
                    Cell::Void | Cell::NS => { /* ignore */ }
                    Cell::NW | Cell::Start | Cell::SW => {
                        /* I manually looked at the input */
                        if open_east {
                            open_east = false;
                            complete_crossings += 1;
                        } else if open_west {
                            open_west = false;
                        } else {
                            open_west = true;
                        }
                    }
                    Cell::NE | Cell::SE => {
                        if open_west {
                            complete_crossings += 1;
                            open_west = false;
                        } else if open_east {
                            open_east = false;
                        } else {
                            open_east = true;
                        }
                    }
                }
            }
            assert!(!open_east);
            assert!(!open_west);

            let complete_crossings = complete_crossings % 2;
            match complete_crossings {
                1 => {
                    inside += 1;
                }
                0 => {}
                _ => {
                    panic!();
                }
            }
        }
    }
    println!("{inside}");
}
