use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;

pub fn run() -> Result<(String, String)> {
    let mut x_values: Vec<i32> = vec![1];
    for line in read_to_string("data/10.txt").unwrap().lines() {
        let prev_value = *x_values.last().unwrap();
        x_values.push(prev_value);
        match &line[0..4] {
            "noop" => (),
            "addx" => x_values.push(prev_value + line[5..].parse::<i32>().unwrap()),
            _ => panic!()
        }
    }

    let part1: i32 = x_values.
        iter().
        skip(19).
        zip(20..).
        step_by(40).
        map(|(i, x)| i * x).
        sum();

    let part2 = (0..240).
        map(|i| i % 40).
        zip(x_values.iter()).
        map(|(col, x)| {
            if (x - 1..=x + 1).contains(&col) { '⬜' }
            else { '⬛' }
        }).
        chunks(40).
        into_iter().
        map(|chunk| chunk.into_iter().join("")).
        join("\n");

    Ok((part1.to_string(), "\n".to_owned() + &part2))
}
