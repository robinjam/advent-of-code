use std::fs::read_to_string;

use anyhow::{Result, Context};

pub fn run() -> Result<(String, String)> {
    let numbers: Vec<_> = read_to_string("data/20.txt")?
        .lines()
        .map(|line| line.parse::<i64>())
        .collect::<Result<_, _>>()?;

    let part1 = solve(&numbers, 1, 1).context("no solution")?;
    let part2 = solve(&numbers, 811589153, 10).context("no solution")?;

    Ok((part1.to_string(), part2.to_string()))
}

fn solve(numbers: &[i64], key: i64, rounds: usize) -> Option<i64> {
    let mut numbers: Vec<_> = numbers.iter().map(|&n| n * key).enumerate().collect();

    for _ in 0..rounds {
        for i in 0..numbers.len() {
            let pos = numbers.iter().position(|&(idx, _n)| idx == i)?;
            let (idx, n) = numbers.remove(pos);
            let new_idx = (pos as i64 + n).rem_euclid(numbers.len() as i64) as usize;
            numbers.insert(new_idx, (idx, n));
        }
    }

    let idx = numbers.iter().position(|&(_idx, n)| n == 0)?;
    Some(
        [1000, 2000, 3000]
            .iter()
            .map(|i| numbers[(idx + i) % numbers.len()].1)
            .sum(),
    )
}
