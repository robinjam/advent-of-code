use std::fs::read_to_string;

use array2d::Array2D;
use pathfinding::prelude::dijkstra;

pub fn run() -> (String, String) {
    let map = load_map(&read_to_string("data/12.txt").unwrap());

    let part1 = steps_required(&map);

    (part1.to_string(), "".into())
}

fn load_map(buf: &str) -> Array2D<Node> {
    Array2D::from_rows(
        &buf.
            lines().
            map(|line|
                line.chars().map(|char|
                    match char {
                        'S' => Node::Start,
                        'E' => Node::End,
                        'a'..='z' => Node::Height(char as i32 - 'a' as i32),
                        _ => panic!()
                    }
                ).collect::<Vec<_>>()
            ).collect::<Vec<_>>()
    )
}

fn steps_required(map: &Array2D<Node>) -> usize {
    let (_, steps) = dijkstra(
        &find_start(map),
        |&pos| {
            neighbours(&map, pos).
                iter().
                map(|&p| (p, 1)).
                collect::<Vec<_>>()
        },
        |&pos| {
            match map[pos] {
                Node::End => true,
                _ => false,
            }
        }
    ).unwrap();

    steps
}

#[derive(Clone, Copy)]
enum Node {
    Start,
    End,
    Height(i32),
}

impl Node {
    fn height(&self) -> i32 {
        match self {
            Node::Start => 0,
            Node::End => 25,
            Node::Height(height) => *height,
        }
    }
}

fn find_start(map: &Array2D<Node>) -> (usize, usize) {
    for x in 0..map.column_len() {
        for y in 0..map.row_len() {
            match map[(x, y)] {
                Node::Start => return (x, y),
                _ => ()
            }
        }
    }
    unreachable!();
}

fn neighbours(map: &Array2D<Node>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let current_height = map[pos].height();

    let mut neighbours = vec![];
    if pos.0 > 0 { neighbours.push((pos.0 - 1, pos.1)); }
    if pos.1 > 0 { neighbours.push((pos.0, pos.1 - 1)); }
    if pos.0 < map.num_rows() - 1 { neighbours.push((pos.0 + 1, pos.1)); }
    if pos.1 < map.num_columns() - 1 { neighbours.push((pos.0, pos.1 + 1)); }

    neighbours.
        iter().
        filter(|&&p|
            map[p].height() - current_height <= 1
        )
        .cloned()
        .collect()
}
