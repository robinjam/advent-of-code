use std::{fs::read_to_string, str::FromStr};

use anyhow::{Result, anyhow, Context, Error};
use array2d::Array2D;
use pathfinding::prelude::dijkstra;

pub fn run() -> (String, String) {
    let InitialState{ heights, start, end } = read_to_string("data/12.txt").unwrap().parse().unwrap();

    let part1 = steps_required(&heights, end, |&pos| { pos == start }).unwrap();
    let part2 = steps_required(&heights, end, |&pos| { heights[pos] == 0 }).unwrap();

    (part1.to_string(), part2.to_string())
}

type Heights = Array2D<i32>;
type Pos = (usize, usize);

struct InitialState {
    heights: Heights,
    start: Pos,
    end: Pos,
}

impl FromStr for InitialState {
    type Err = Error;

    fn from_str(buf: &str) -> Result<Self> {
        let mut start: Option<Pos> = None;
        let mut end: Option<Pos> = None;

        let rows: Vec<Vec<i32>> = buf.lines().enumerate().map(|(row_idx, line)|
            line.chars().enumerate().map(|(col_idx, char)| {
                match char {
                    'S' => {
                        match start {
                            Some(..) => Err(anyhow!("multiple start squares found in input")),
                            None => {
                                start = Some((row_idx, col_idx));
                                Ok(0)
                            }
                        }
                    },
                    'E' => {
                        match end {
                            Some(..) => Err(anyhow!("multiple end squares found in input")),
                            None => {
                                end = Some((row_idx, col_idx));
                                Ok(25)
                            }
                        }
                    },
                    'a'..='z' => Ok(char as i32 - 'a' as i32),
                    _ => Err(anyhow!("invalid square: {}", char))
                }
            }).collect()
        ).collect::<Result<_>>()?;

        let heights = Array2D::from_rows(&rows);
        let start = start.context("can't find start square in input")?;
        let end = end.context("can't find end square in input")?;

        Ok(InitialState{ heights, start, end })
    }
}

fn steps_required<F>(heights: &Heights, start: Pos, is_goal: F) -> Option<usize>
    where F: Fn(&Pos) -> bool
{
    let (_, steps) = dijkstra(
        &start,
        |&pos| {
            let mut neighbours = vec![];
            if pos.0 > 0                         { neighbours.push(((pos.0 - 1, pos.1    ), 1)); }
            if pos.1 > 0                         { neighbours.push(((pos.0    , pos.1 - 1), 1)); }
            if pos.0 < heights.num_rows() - 1    { neighbours.push(((pos.0 + 1, pos.1    ), 1)); }
            if pos.1 < heights.num_columns() - 1 { neighbours.push(((pos.0    , pos.1 + 1), 1)); }
            neighbours.retain(|next| heights[pos] - heights[next.0] <= 1);
            neighbours
        },
        is_goal
    )?;

    Some(steps)
}
