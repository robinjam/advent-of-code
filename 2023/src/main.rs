use std::fs::read_to_string;

mod day01;
mod day02;
mod day04;
mod day08;

fn main() {
    let (part1, part2) = day08::day08(&read_to_string("inputs/08.txt").unwrap());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
