use std::{fs::read_to_string, collections::HashSet};

use anyhow::Result;

type Vec2 = (i32, i32);

fn next_tail_position(mut tail: Vec2, head: Vec2) -> Vec2 {
    match (tail.0 - head.0, tail.1 - head.1) {
        (-1..=1, -1..=1) => (),

        (-2,  0) => tail.0 += 1,
        ( 2,  0) => tail.0 -= 1,
        ( 0, -2) => tail.1 += 1,
        ( 0,  2) => tail.1 -= 1,

        (x, y) => {
            tail.0 += if x < 0 { 1 } else { -1 };
            tail.1 += if y < 0 { 1 } else { -1 };
        }
    }
    tail
}

fn make_moves(rope: &mut [Vec2], buf: &str) -> usize {
    let mut tail_positions: HashSet<Vec2> = [(0, 0)].into();

    for line in buf.lines() {
        let (dir, count) = line.split_once(" ").unwrap();
        let (x, y) = match dir {
            "L" => (-1,  0),
            "R" => ( 1,  0),
            "D" => ( 0, -1),
            "U" => ( 0,  1),
            _ => panic!()
        };
        for _ in 0..count.parse().unwrap() {
            rope[0].0 += x;
            rope[0].1 += y;
            for i in 1..rope.len() {
                rope[i] = next_tail_position(rope[i], rope[i - 1]);
            }
            tail_positions.insert(*rope.last().unwrap());
        }
    }

    tail_positions.len()
}

pub fn run() -> Result<(String, String)> {
    let buf = read_to_string("data/09.txt").unwrap();

    let part1 = make_moves(&mut [Vec2::default(); 2], &buf);
    let part2 = make_moves(&mut [Vec2::default(); 10], &buf);

    Ok((part1.to_string(), part2.to_string()))
}
