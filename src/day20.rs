use std::fs::read_to_string;

use anyhow::{Result, Context};

pub fn run() -> Result<(String, String)> {
    let numbers: Vec<_> = read_to_string("data/20.txt")?
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<_, _>>()?;

    let part1 = solve(&numbers).context("no solution")?;

    Ok((part1.to_string(), "TODO".into()))
}

fn solve(numbers: &[i32]) -> Option<i32> {
    let mut numbers: Vec<_> = numbers.iter().cloned().enumerate().collect();

    for i in 0..numbers.len() {
        let pos = numbers.iter().position(|&(idx, _n)| idx == i)?;
        let (idx, n) = numbers.remove(pos);
        let new_idx = (pos as i32 + n).rem_euclid(numbers.len() as i32) as usize;
        numbers.insert(new_idx, (idx, n));
    }

    let idx = numbers.iter().position(|&(_idx, n)| n == 0)?;
    Some(
        [1000, 2000, 3000]
            .iter()
            .map(|i| numbers[(idx + i) % numbers.len()].1)
            .sum(),
    )
}
