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

    (part1.to_string(), "".into())
}
