use std::{str::FromStr, collections::HashMap};

use anyhow::{Result, Error, Context};

pub fn run() -> Result<(String, String)> {
    let cave: Cave = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9".parse()?;

    for row in 0..=9 {
        for col in 494..=503 {
            print!("{}", if *cave.rocks.get(&Pos(row, col)).unwrap_or(&false) { "#" } else { "." });
        }
        println!()
    }

    Ok(("TODO".into(), "TODO".into()))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

impl FromStr for Pos {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(",").context("position does not contain comma")?;
        Ok(Pos(a.parse()?, b.parse()?))
    }
}

struct Cave {
    rocks: HashMap<Pos, bool>,
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks: HashMap<Pos, bool> = HashMap::new();

        for line in s.lines() {
            let positions: Vec<Pos> = line
                .split(" -> ")
                .map(|pos| Ok(pos.parse()?))
                .collect::<Result<_>>()?;
            
            for window in positions.windows(2) {
                if let [a, b] = window {
                    let min_x = a.0.min(b.0);
                    let max_x = a.0.max(b.0);
                    let min_y = a.1.min(b.1);
                    let max_y = a.1.max(b.1);
                    for col in min_x..=max_x {
                        for row in min_y..=max_y {
                            rocks.insert(Pos(row, col), true);
                        }
                    }
                }
            }
        }

        Ok(Cave{ rocks })
    }
}
