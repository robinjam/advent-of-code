use std::fs::read_to_string;

mod day01;
mod day02;

fn main() {
    let (part1, part2) = day02::day02(&read_to_string("inputs/02.txt").unwrap());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
