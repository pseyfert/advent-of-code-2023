use serde::{de, Deserialize};
use std::io::BufRead;

#[derive(Debug)]
struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

#[derive(Debug)]
struct Draw {
    pub blue: u32,
    pub red: u32,
    pub green: u32,
}

impl std::ops::Add for Draw {
    type Output = Draw;
    fn add(self, other: Draw) -> Draw {
        Draw {
            blue: self.blue + other.blue,
            red: self.red + other.red,
            green: self.green + other.green,
        }
    }
}

impl Draw {
    fn max() -> Draw {
        Draw {
            blue: u32::MAX,
            red: u32::MAX,
            green: u32::MAX,
        }
    }
    fn null() -> Draw {
        Draw {
            blue: 0,
            red: 0,
            green: 0,
        }
    }
}

impl std::iter::Sum for Draw {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Draw::null(), |acc, x| acc + x)
    }
}

impl std::cmp::PartialEq for Draw {
    fn eq(&self, other: &Draw) -> bool {
        if self.red == other.red && self.blue == other.blue && self.green == other.green {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for Draw {
    fn partial_cmp(&self, other: &Draw) -> Option<std::cmp::Ordering> {
        if self.red == other.red && self.blue == other.blue && self.green == other.green {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.red <= other.red && self.blue <= other.blue && self.green <= other.green {
            return Some(std::cmp::Ordering::Less);
        }
        if self.red >= other.red && self.blue >= other.blue && self.green >= other.green {
            return Some(std::cmp::Ordering::Greater);
        }
        return None;
    }
}

fn parse_draws(s: &str) -> Vec<Draw> {
    s.split(';')
        .map(|draw_string| {
            draw_string
                .split(',')
                .filter_map(|single_color| -> Option<Draw> {
                    let mut it = single_color.split(' ');
                    let (Some(""), Some(count), Some(color), None) =
                        (it.next(), it.next(), it.next(), it.next())
                    else {
                        println!("single draw {single_color} didn't look right");
                        return None;
                    };
                    let Ok(count) = count.parse::<u32>() else {
                        println!("count for this color is not a number");
                        return None;
                    };
                    match color {
                        "green" => Some(Draw {
                            blue: 0,
                            red: 0,
                            green: count,
                        }),
                        "blue" => Some(Draw {
                            blue: count,
                            red: 0,
                            green: 0,
                        }),
                        "red" => Some(Draw {
                            blue: 0,
                            red: count,
                            green: 0,
                        }),
                        _ => {
                            println!("unexpected color");
                            None
                        }
                    }
                })
                // .inspect(|d| println!("looks like we drew {d:?}"))
                .sum::<Draw>()
        })
        .collect()
}

fn lower_bound(input: &Vec<Draw>) -> Draw {
    let rv = input.iter().fold(Draw::null(), |acc, x| Draw {
        blue: std::cmp::max(acc.blue, x.blue),
        red: std::cmp::max(acc.red, x.red),
        green: std::cmp::max(acc.green, x.green),
    });
    // println!("for the game with {input:?}, the lower bound of needed cubes is {rv:?}");
    rv
}

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Game, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let current_line: &str = de::Deserialize::deserialize(deserializer)?;
        let mut colon = current_line.split(':');
        let (Some(game_string), Some(draws_string), None) =
            (colon.next(), colon.next(), colon.next())
        else {
            return Err(de::Error::custom("Couldn't split line by one ':'"));
        };

        let Ok(game_re) = regex::Regex::new(r"^Game (?P<id>[0-9]+)") else {
            return Err(de::Error::custom("internal regex didn't compile"));
        };

        let Some(parsed_game) = game_re.captures(game_string) else {
            return Err(de::Error::custom("Regex didn't match"));
        };

        let Some(id) = parsed_game
            .name("id")
            .and_then(|s| s.as_str().parse::<u32>().ok())
        else {
            return Err(de::Error::custom(
                "Didn't find Regex group or failed to parse integer",
            ));
        };

        Ok(Game {
            id,
            draws: parse_draws(draws_string),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
}

fn main() {
    // TODO: a serde "every line is an element" would be nicer
    let input: Vec<Game> = std::io::BufReader::new(std::fs::File::open("../input").unwrap())
        .lines()
        .map(|l| serde_plain::from_str(&l.unwrap()).unwrap())
        .collect();

    let bag_content = Draw {
        blue: 14,
        red: 12,
        green: 13,
    };
    let part_1_result: u32 = input
        .iter()
        .filter(|game| bag_content >= lower_bound(&game.draws))
        .map(|game| game.id)
        .sum();

    println!("part 1: {part_1_result}");

    let part_2_result = input
        .iter()
        .map(|game| {
            let lb = lower_bound(&game.draws);
            lb.blue * lb.red * lb.green
        })
        .sum::<u32>();

    println!("part 2: {part_2_result}");
}
