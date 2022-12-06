mod day01;
mod day02;
mod day03;

fn main() {
    let days: Vec<fn () -> (i32, i32)> = vec![
        day01::run,
        day02::run,
        day03::run,
    ];
    
    for (i, f) in days.iter().enumerate() {
        let (part1, part2) = f();
        println!("Day {}", i + 1);
        println!("  Part 1: {}", part1);
        println!("  Part 2: {}", part2);
    }
}
