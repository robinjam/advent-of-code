use std::fs::read_to_string;

use anyhow::{Error, Result, anyhow};
use array2d::Array2D;
use pathfinding::prelude::dijkstra;

pub fn run() -> (String, String) {
    let map = load_map(&read_to_string("data/12.txt").unwrap()).unwrap();

    let part1 = steps_required(&map, |&pos| {
        match map[pos] {
            Square::Start => true,
            _ => false,
        }
    }).unwrap();

    let part2 = steps_required(&map, |&pos| {
        match map[pos] {
            Square::Height(0) => true,
            _ => false,
        }
    }).unwrap();

    (part1.to_string(), part2.to_string())
}

type Pos = (usize, usize);

fn load_map(buf: &str) -> Result<Array2D<Square>> {
    let rows: Vec<Vec<Square>> = buf.lines().map(|line|
        line.chars().map(|char| char.try_into() ).collect()
    ).collect::<Result<_>>()?;

    Ok(Array2D::from_rows(&rows))
}

fn steps_required<F>(map: &Array2D<Square>, goal: F) -> Option<usize>
    where F: Fn(&Pos) -> bool
{
    let (_, steps) = dijkstra(
        &find_end(map)?,
        |&pos| {
            neighbours(&map, pos).
                iter().
                map(|&p| (p, 1)).
                collect::<Vec<_>>()
        },
        goal
    )?;

    Some(steps)
}

#[derive(Clone, Copy)]
enum Square {
    Start,
    End,
    Height(i32),
}

impl Square {
    fn height(&self) -> i32 {
        match self {
            Square::Start => 0,
            Square::End => 25,
            Square::Height(height) => *height,
        }
    }
}

impl TryFrom<char> for Square {
    type Error = Error;

    fn try_from(value: char) -> Result<Self> {
        match value {
            'S' => Ok(Square::Start),
            'E' => Ok(Square::End),
            'a'..='z' => Ok(Square::Height(value as i32 - 'a' as i32)),
            _ => Err(anyhow!("Invalid square: {}", value))
        }
    }
}

fn find_end(map: &Array2D<Square>) -> Option<Pos> {
    for col in 0..map.num_columns() {
        for row in 0..map.num_rows() {
            match map[(row, col)] {
                Square::End => return Some((row, col)),
                _ => ()
            }
        }
    }
    None
}

fn neighbours(map: &Array2D<Square>, pos: Pos) -> Vec<Pos> {
    let height = map[pos].height();

    let mut neighbours = vec![];
    if pos.0 > 0 { neighbours.push((pos.0 - 1, pos.1)); }
    if pos.1 > 0 { neighbours.push((pos.0, pos.1 - 1)); }
    if pos.0 < map.num_rows() - 1 { neighbours.push((pos.0 + 1, pos.1)); }
    if pos.1 < map.num_columns() - 1 { neighbours.push((pos.0, pos.1 + 1)); }

    neighbours.
        iter().
        filter(|&&p| height - map[p].height() <= 1).
        cloned().
        collect()
}
