use std::fs::read_to_string;

mod day01;
mod day02;
mod day04;

fn main() {
    let (part1, part2) = day04::day04(&read_to_string("inputs/04.txt").unwrap());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
