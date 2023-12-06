use std::fs::read_to_string;

use anyhow::{Result, Context};
use serde::Deserialize;

pub fn run() -> Result<(String, String)> {
    let buf = read_to_string("data/13.txt")?;

    Ok((
        part1(&buf)?.to_string(),
        part2(&buf)?.to_string()
    ))
}

fn part1(buf: &str) -> Result<usize> {
    let packet_pairs: Vec<(Packet, Packet)> = buf
        .split("\n\n")
        .map(|chunk| Ok((
            serde_json::from_str(chunk.lines().nth(0).context("chunk has fewer than 2 lines")?)?,
            serde_json::from_str(chunk.lines().nth(1).context("chunk has fewer than 2 lines")?)?
        )))
        .collect::<Result<_>>()?;
    
    Ok(packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, (a, b))| a <= b)
        .map(|(i, _)| i + 1)
        .sum()
    )
}

fn part2(buf: &str) -> Result<usize> {
    let divider_a = Packet::List(vec![Packet::List(vec![Packet::Integer(2)])]);
    let divider_b = Packet::List(vec![Packet::List(vec![Packet::Integer(6)])]);

    let mut all_packets: Vec<Packet> = buf
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Ok(serde_json::from_str(line)?))
        .collect::<Result<_>>()?;

    all_packets.push(divider_a.clone());
    all_packets.push(divider_b.clone());
    all_packets.sort();

    let divider_a_position = all_packets.iter().position(|p| *p == divider_a).unwrap();
    let divider_b_position = all_packets.iter().position(|p| *p == divider_b).unwrap();
    Ok((divider_a_position + 1) * (divider_b_position + 1))
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
