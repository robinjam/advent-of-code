use std::fs::read_to_string;

use anyhow::{Result, Context};
use serde::Deserialize;

pub fn run() -> Result<(String, String)> {
    let packet_pairs: Vec<(Packet, Packet)> = read_to_string("data/13.txt")?
        .split("\n\n")
        .map(|chunk| Ok((
            serde_json::from_str(chunk.lines().nth(0).context("chunk has fewer than 2 lines")?)?,
            serde_json::from_str(chunk.lines().nth(1).context("chunk has fewer than 2 lines")?)?
        )))
        .collect::<Result<_>>()?;
    
    let part1: usize = packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a <= b)
        .map(|(i, _)| i + 1)
        .sum();

    Ok((part1.to_string(), "".to_owned()))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    List(Vec<Packet>),
    Integer(i32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.cmp(b),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
            (a @ Packet::Integer(_), b @ Packet::List(_)) => Packet::List(vec![a.clone()]).cmp(b),
            (a @ Packet::List(_), b @ Packet::Integer(_)) => a.cmp(&Packet::List(vec![b.clone()])),
        }
    }
}
