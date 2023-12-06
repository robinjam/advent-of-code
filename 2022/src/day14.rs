use std::{str::FromStr, collections::HashSet, ops::Add, fs::read_to_string};

use anyhow::{Result, Error, Context};

pub fn run() -> Result<(String, String)> {
    let cave: Cave = read_to_string("data/14.txt")?.parse()?;
    
    let part1 = cave.clone().num_moves(false)?;
    let part2 = cave.num_moves(true)?;

    Ok((part1.to_string(), part2.to_string()))
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

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Clone)]
struct Cave {
    blocked: HashSet<Pos>,
}

impl Cave {
    fn height(&self) -> Option<i32> {
        self.blocked.iter().map(|pos| pos.0).max()
    }

    fn num_moves(mut self, has_floor: bool) -> Result<i32> {
        let height = self.height().context("cave is empty")?;

        let mut count = 0;
        'outer: loop {
            let mut pos = Pos(0, 500);
            loop {
                let down = pos + Pos(1, 0);
                let down_left = pos + Pos(1, -1);
                let down_right = pos + Pos(1, 1);
                if has_floor && pos.0 == height + 1 {
                    break;
                }
                else if !has_floor && pos.0 >= height {
                    break 'outer;
                }
                else if !self.blocked.contains(&down) {
                    pos = down;
                }
                else if !self.blocked.contains(&down_left) {
                    pos = down_left;
                }
                else if !self.blocked.contains(&down_right) {
                    pos = down_right;
                }
                else if pos == Pos(0, 500) {
                    count += 1;
                    break 'outer;
                }
                else {
                    break;
                }
            }
            self.blocked.insert(pos);
            count += 1;
        }

        Ok(count)
    }
}

impl FromStr for Cave {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocked: HashSet<Pos> = HashSet::new();

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
                            blocked.insert(Pos(row, col));
                        }
                    }
                }
            }
        }

        Ok(Cave{ blocked })
    }
}
