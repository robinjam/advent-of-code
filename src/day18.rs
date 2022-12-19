use std::{collections::HashSet, fs::read_to_string, ops::Add, str::FromStr};

use anyhow::{Context, Error, Result};

pub fn run() -> Result<(String, String)> {
    let scan: Scan = read_to_string("data/18.txt")?.parse()?;

    let part1 = scan.surface_area();

    Ok((part1.to_string(), "TODO".into()))
}

struct Scan {
    cubes: HashSet<Pos>,
}

impl Scan {
    fn surface_area(&self) -> usize {
        self.cubes
            .iter()
            .map(|cube| {
                cube.neighbours()
                    .iter()
                    .filter(|neighbour| !self.cubes.contains(neighbour))
                    .count()
            })
            .sum()
    }
}

impl FromStr for Scan {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let cubes = s.lines().map(|line| line.parse()).collect::<Result<_>>()?;

        Ok(Scan { cubes })
    }
}

#[derive(Eq, Hash, PartialEq)]
struct Pos(i32, i32, i32);

impl Pos {
    fn neighbours(&self) -> [Pos; 6] {
        [
            self + &Pos( 0,  0,  1),
            self + &Pos( 0,  0, -1),
            self + &Pos( 0,  1,  0),
            self + &Pos( 0, -1,  0),
            self + &Pos( 1,  0,  0),
            self + &Pos(-1,  0,  0),
        ]
    }
}

impl Add for &Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Pos {
        Pos(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl FromStr for Pos {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let err_string = format!("invalid position: {}", s);
        let (x, yz) = s.split_once(",").context(err_string.clone())?;
        let (y, z) = yz.split_once(",").context(err_string)?;
        Ok(Pos(x.parse()?, y.parse()?, z.parse()?))
    }
}
