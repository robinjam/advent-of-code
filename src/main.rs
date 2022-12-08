mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() {
    let days: Vec<fn () -> (String, String)> = vec![
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
    ];
    
    for (i, f) in days.iter().enumerate() {
        let (part1, part2) = f();
        println!("Day {}", i + 1);
        println!("  Part 1: {}", part1);
        println!("  Part 2: {}", part2);
    }
}
