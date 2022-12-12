use std::fs::read_to_string;

use anyhow::Result;
use array2d::Array2D;
use itertools::Itertools;

fn is_visible(trees: &Array2D<u32>, (x, y): (usize, usize)) -> bool {
    let xn = trees.column_len();
    let yn = trees.row_len();
    let tree = trees[(x, y)];
    (    0..x ).all(|i| trees[(i, y)] < tree) ||
    (x + 1..xn).all(|i| trees[(i, y)] < tree) ||
    (    0..y ).all(|i| trees[(x, i)] < tree) ||
    (y + 1..yn).all(|i| trees[(x, i)] < tree)
}

fn scenic_score(trees: &Array2D<u32>, (x, y): (usize, usize)) -> i32 {
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
    for i in x + 1..trees.column_len() {
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
    for i in y + 1..trees.row_len() {
        down += 1;
        if trees[(x, i)] >= tree {
            break;
        }
    }

    up * down * left * right
}

pub fn run() -> Result<(String, String)> {
    let rows: Vec<Vec<u32>> = read_to_string("data/08.txt").
        unwrap().
        lines().
        map(|line|
            line.
                chars().
                map(|c| c.to_digit(10).unwrap()).
                collect()
        ).
        collect();

    let trees = Array2D::from_rows(&rows);

    let all_indices = (0..trees.column_len()).
        cartesian_product(0..trees.row_len());

    let part1 = all_indices.clone().
        filter(|&i| is_visible(&trees, i)).
        count();

    let part2 = all_indices.
        map(|i| scenic_score(&trees, i)).
        max().
        unwrap();

    Ok((part1.to_string(), part2.to_string()))
}
