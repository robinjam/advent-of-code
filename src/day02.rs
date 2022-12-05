use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

fn move_from_char(c: char) -> Move {
    match c {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => panic!()
    }
}

fn moves_from_line(line: String) -> (Move, Move) {
    let move_from_nth_char = |n| move_from_char(line.chars().nth(n).unwrap());
    (move_from_nth_char(0), move_from_nth_char(2))
}

#[test]
fn test_moves_from_line() {
    assert_eq!((Move::Rock, Move::Paper), moves_from_line("A Y".into()));
    assert_eq!((Move::Paper, Move::Rock), moves_from_line("B X".into()));
    assert_eq!((Move::Scissors, Move::Scissors), moves_from_line("C Z".into()));
}

fn score(moves: (Move, Move)) -> i32 {
    let (theirs, mine) = moves;

    let shape_score = match mine {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3
    };

    let outcome_score = match (mine, theirs) {
        (Move::Rock, Move::Scissors) => 6,
        (Move::Paper, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 6,

        (Move::Rock, Move::Rock) => 3,
        (Move::Paper, Move::Paper) => 3,
        (Move::Scissors, Move::Scissors) => 3,

        (Move::Rock, Move::Paper) => 0,
        (Move::Paper, Move::Scissors) => 0,
        (Move::Scissors, Move::Rock) => 0,
    };

    shape_score + outcome_score
}

#[test]
fn test_score() {
    assert_eq!(8, score((Move::Rock, Move::Paper)));
    assert_eq!(1, score((Move::Paper, Move::Rock)));
    assert_eq!(6, score((Move::Scissors, Move::Scissors)));
}

pub fn run() {
    let file = File::open("data/02.txt").unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let score: i32 = lines.map(moves_from_line).map(score).sum();

    println!("Day 02");
    println!("  Part 1: {}", score);
}
