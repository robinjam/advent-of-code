use std::fs::read_to_string;

use anyhow::Result;

pub fn run() -> Result<(String, String)> {
    let mut sums: Vec<i32> =
        read_to_string("data/01.txt").
        unwrap().
        trim_end().
        split("\n\n").
        map(|block| {
            block.
                lines().
                map(|line| line.parse::<i32>().unwrap()).
                sum()
        }).
        collect();

    sums.sort_unstable_by(|a, b| b.cmp(a));

    let part1: i32 = sums[0];
    let part2: i32 = sums[0..3].iter().sum();

    Ok((part1.to_string(), part2.to_string()))
}
