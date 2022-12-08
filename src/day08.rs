use std::fs::read_to_string;

use array2d::Array2D;

fn is_visible(trees: &Array2D<char>, x: usize, y: usize) -> bool {
    let xn = trees.column_len();
    let yn = trees.row_len();
    let tree = trees[(x, y)];
    (    0..x ).all(|i| trees[(i, y)] < tree) ||
    (x + 1..xn).all(|i| trees[(i, y)] < tree) ||
    (    0..y ).all(|i| trees[(x, i)] < tree) ||
    (y + 1..yn).all(|i| trees[(x, i)] < tree)
}

fn scenic_score(trees: &Array2D<char>, x: usize, y: usize) -> i32 {
    let xn = trees.column_len();
    let yn = trees.row_len();
    let tree = trees[(x, y)];
    let mut up = 0;
    let mut down = 0;
    let mut left = 0;
    let mut right = 0;

    for i in (0..x).rev() {
        left += 1;
        if trees[(i, y)] >= tree {
            break;
        }
    }
    for i in x + 1..xn {
        right += 1;
        if trees[(i, y)] >= tree {
            break;
        }
    }
    for i in (0..y).rev() {
        up += 1;
        if trees[(x, i)] >= tree {
            break;
        }
    }
    for i in y + 1..yn {
        down += 1;
        if trees[(x, i)] >= tree {
            break;
        }
    }

    up * down * left * right
}

pub fn run() -> (String, String) {
    let rows: Vec<Vec<char>> = read_to_string("data/08.txt").
        unwrap().
        lines().
        map(|line| line.chars().collect()).
        collect();

    let trees = Array2D::from_rows(rows.as_slice());

    let mut part1 = 0;
    for x in 0..trees.column_len() {
        for y in 0..trees.row_len() {
            if is_visible(&trees, x, y) {
                part1 += 1;
            }
        }
    }

    let mut scores: Vec<i32> = vec![];
    for x in 0..trees.column_len() {
        for y in 0..trees.row_len() {
            scores.push(scenic_score(&trees, x, y));
        }
    }
    let part2 = scores.iter().max().unwrap();

    (part1.to_string(), part2.to_string())
}
