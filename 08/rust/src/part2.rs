use std::collections::HashMap;

#[derive(Debug, PartialOrd, Eq, PartialEq, Clone, Copy)]
pub struct InstState(pub (usize, usize));

impl Ord for InstState {
    fn cmp(&self, other: &InstState) -> std::cmp::Ordering {
        match self.0 .1.cmp(&other.0 .1) {
            std::cmp::Ordering::Equal => self.0 .0.cmp(&other.0 .0),
            o => o,
        }
    }
}

pub struct CycleDetector {
    dests_seen: HashMap<[char; 3], Vec<InstState>>,
}

#[derive(Debug)]
pub struct Cycles {
    pub single_visits: Vec<InstState>,
    pub cycle_visits: Vec<InstState>,
    pub cycle_len: usize,
}

impl CycleDetector {
    pub fn new() -> Self {
        Self {
            dests_seen: HashMap::new(),
        }
    }

    // returns after how many runs through the instruction list we reach a destination again with
    // the same "instruction pointer".
    pub fn add_dest(&mut self, loc: [char; 3], inst_state: InstState) -> Option<Cycles> {
        match self.dests_seen.get_mut(&loc) {
            None => {
                self.dests_seen.insert(loc, vec![inst_state]);
                None
            }
            Some(v) => match v.iter().find(|is| is.0 .0 == inst_state.0 .0) {
                None => {
                    v.push(inst_state);
                    None
                }
                Some(e) => {
                    let e = e.clone();
                    let dests: Vec<_> = (&mut self.dests_seen).values().flatten().collect();
                    let (s, c) = dests.into_iter().partition(|v| *v < &e);

                    Some(Cycles {
                        single_visits: s,
                        cycle_visits: c,
                        cycle_len: inst_state.0 .1 - e.0 .1,
                    })
                }
            },
        }
    }
}
