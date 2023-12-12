use std::fs::read_to_string;

mod day01;
mod day02;
mod day04;
mod day08;
mod day11;

fn main() {
    let (part1, part2) = day11::day11(&read_to_string("inputs/11.txt").unwrap());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
