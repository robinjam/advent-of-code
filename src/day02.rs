use std::fs::read_to_string;

fn score_1(line: &str) -> i32 {
    match line {
        "B X" => 1,
        "C Y" => 2,
        "A Z" => 3,
        "A X" => 4,
        "B Y" => 5,
        "C Z" => 6,
        "C X" => 7,
        "A Y" => 8,
        "B Z" => 9,
        _ => panic!()
    }
}

fn score_2(line: &str) -> i32 {
    match line {
        "B X" => 1,
        "C X" => 2,
        "A X" => 3,
        "A Y" => 4,
        "B Y" => 5,
        "C Y" => 6,
        "C Z" => 7,
        "A Z" => 8,
        "B Z" => 9,
        _ => panic!()
    }
}

pub fn run() {
    let buf = read_to_string("data/02.txt").unwrap();

    let part_1_score: i32 = buf.lines().map(score_1).sum();
    let part_2_score: i32 = buf.lines().map(score_2).sum();

    println!("Day 02");
    println!("  Part 1: {}", part_1_score);
    println!("  Part 2: {}", part_2_score);
}
