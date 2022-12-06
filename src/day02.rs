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

pub fn run() -> (i32, i32) {
    let buf = read_to_string("data/02.txt").unwrap();

    (
        buf.lines().map(score_1).sum(),
        buf.lines().map(score_2).sum()
    )
}
