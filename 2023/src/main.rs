use std::fs::read_to_string;

mod day01;

fn main() {
    let (part1, part2) = day01::day01(&read_to_string("inputs/01.txt").unwrap());
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
