use std::{collections::HashSet, fs::read_to_string, ops::Add, str::FromStr};

use anyhow::{Context, Error, Result};

pub fn run() -> Result<(String, String)> {
    let scan: Scan = read_to_string("data/18.txt")?.parse()?;

    let part1 = scan.surface_area();
    let part2 = scan.with_holes_filled().surface_area();

    Ok((part1.to_string(), part2.to_string()))
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

    fn with_holes_filled(&self) -> Self {
        // Perform flood fill of (padded) bounding box to discover air cubes, then invert
        let max_x = *self.cubes.iter().map(|Pos(x, _, _)| x).max().unwrap_or(&0) + 1;
        let max_y = *self.cubes.iter().map(|Pos(_, y, _)| y).max().unwrap_or(&0) + 1;
        let max_z = *self.cubes.iter().map(|Pos(_, _, z)| z).max().unwrap_or(&0) + 1;
        let mut queue: Vec<Pos> = vec![Pos(0, 0, 0)];
        let mut air_cubes: HashSet<Pos> = HashSet::new();
        while let Some(pos) = queue.pop() {
            if !self.cubes.contains(&pos) && !air_cubes.contains(&pos) {
                for neighbour in pos.neighbours() {
                    if neighbour.0 >= 0
                        && neighbour.0 <= max_x
                        && neighbour.1 >= 0
                        && neighbour.1 <= max_y
                        && neighbour.2 >= 0
                        && neighbour.2 <= max_z
                    {
                        queue.push(neighbour);
                    }
                }
                air_cubes.insert(pos);
            }
        }

        let mut cubes: HashSet<Pos> = HashSet::new();
        for x in 0..=max_x {
            for y in 0..=max_y {
                for z in 0..=max_z {
                    let pos = Pos(x, y, z);
                    if !air_cubes.contains(&pos) {
                        cubes.insert(pos);
                    }
                }
            }
        }
        Self { cubes }
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
